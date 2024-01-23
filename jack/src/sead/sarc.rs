use {
    crate::{IntoBytes, JackFile, Pathed},
    byteorder::{BigEndian, LittleEndian, ReadBytesExt},
    log::info,
    macros::fail,
    std::{
        collections::BTreeMap,
        io::{BufRead, BufReader, Cursor, Error, Seek, SeekFrom},
        str::from_utf8,
    },
};

/// SARC Archive File
pub struct Sarc {
    path: String,
    files: BTreeMap<u32, Vec<SarcInnerFile>>,
    multiplier: u32,
    version: u16,
}

impl Sarc {
    /// Adds a new file to this [`Sarc`] Archive
    /// The `named` field determines whether the file's actual name will be stored in the archive's SFNT Filename Table.
    /// This can usually be set to false safely, but a small number of files do need this to deal with Hash collisions.
    #[allow(unused)]
    pub(crate) fn create(&mut self, filename: &str, data: Vec<u8>, named: bool) {
        if self.read(filename).is_some() {
            fail!("File '{}' with matching Hash already exists in SZS Archive: '{}'", filename, self.path);
        }

        self.files.insert(
            self.calculate_hash(filename),
            vec![SarcInnerFile { filename: if named { Some(filename.to_owned()) } else { None }, data }],
        );
    }

    /// Gets a file with the given `filename` from within this [`Sarc`] Archive. Panics if the file does not exist.
    #[allow(unused)]
    pub(crate) fn read(&self, filename: &str) -> Option<Vec<u8>> {
        if let Some(files) = self.files.get(&self.calculate_hash(filename)) {
            if files.len() == 1 {
                Some(files.get(0).unwrap().data.clone())
            } else {
                Some(
                    files
                        .iter()
                        .find(|&file| {
                            if let Some(filename_inner) = file.filename.clone() {
                                filename == filename_inner
                            } else {
                                false
                            }
                        })
                        .unwrap_or_else(|| {
                            panic!("File with hash collision did not have matching filename: {}", filename)
                        })
                        .data
                        .clone(),
                )
            }
        } else {
            None
        }
    }

    /// Updates a file within this [`Sarc`] Archive
    #[allow(unused)]
    pub(crate) fn update(&mut self, filename: &str, data: Vec<u8>) {
        if let Some(files) = self.files.get_mut(&self.calculate_hash(filename)) {
            if files.len() == 1 {
                files.get_mut(0).unwrap().data = data;
            } else {
                files
                    .iter_mut()
                    .find(|file| {
                        if let Some(filename_inner) = file.filename.clone() {
                            filename == filename_inner
                        } else {
                            false
                        }
                    })
                    .unwrap_or_else(|| panic!("File with hash collision did not have matching filename: {}", filename))
                    .data = data;
            }
        } else {
            fail!("Could not update file '{}' in SARC archive '{}': File doesn't exist.", filename, self.path);
        }
    }

    /// Deletes a file with the given `filename` from within this [`Sarc`] Archive. Panics if the file does not exist.
    #[allow(unused)]
    pub(crate) fn delete(&mut self, filename: &str) {
        let filename_hash = self.calculate_hash(filename);
        if let Some(files) = self.files.get_mut(&filename_hash) {
            if files.len() == 1 {
                self.files.remove(&filename_hash);
                return; // success
            } else if let Some(index) = files.iter().position(|file| file.filename.as_deref() == Some(filename)) {
                files.remove(index);
                return; // success
            }
        }
        fail!("Could not delete file '{}' in SARC Archive '{}': File doesn't exist", filename, self.path);
    }

    /// Creates a representation of a [`Sarc`] Archive from the given file `path` and array of `bytes`.
    pub(crate) fn from(path: &str, bytes: Box<[u8]>) -> Result<Self, Error> {
        let mut buf = BufReader::new(Cursor::new(&bytes));

        // SARC Header
        assert_eq!(&buf.read_u32::<BigEndian>()?.to_be_bytes(), b"SARC", "SARC Magic: {}", path);
        assert_eq!(buf.read_u16::<LittleEndian>()?, 0x14, "SARC Header Size: {}", path);
        assert_eq!(buf.read_u16::<BigEndian>()?, 0xFFFE);

        buf.seek(SeekFrom::Start(0xC))?;
        let offset_to_data = buf.read_u32::<LittleEndian>()? as usize;
        let version = buf.read_u16::<BigEndian>()?;

        // SFAT Section
        buf.seek(SeekFrom::Start(0x14))?;
        assert_eq!(&buf.read_u32::<BigEndian>()?.to_be_bytes(), b"SFAT", "SFAT Magic: {}", path);
        assert_eq!(buf.read_u16::<LittleEndian>()?, 0xC, "SFAT Header Size: {}", path);
        let num_files = buf.read_u16::<LittleEndian>()?;
        let multiplier = buf.read_u32::<LittleEndian>()?;

        // FAT Entries
        struct FatEntry {
            filename_hash: u32,
            filename_hash_count: u8,
            filename_table_offset: u32,
            file_start: u32,
            file_end: u32,
        }

        let mut fat_entries = Vec::new();
        for _ in 0..num_files {
            let filename_hash = buf.read_u32::<LittleEndian>()?;
            let filename_attributes = buf.read_u32::<LittleEndian>()?;
            let file_start = buf.read_u32::<LittleEndian>()?;
            let file_end = buf.read_u32::<LittleEndian>()?;

            // First byte of attrs is the hash collision count, remaining 3 are the offset into the filename table
            let filename_hash_count = (&filename_attributes >> 0x18) as u8;
            let filename_table_offset = filename_attributes & 0x00FFFFFF;

            fat_entries.push(FatEntry {
                filename_hash,
                filename_hash_count,
                filename_table_offset,
                file_start,
                file_end,
            });
        }

        // SFNT Section
        let filename_table_start = 0x28 + (num_files * 0x10) as u64;

        // Data Section
        let mut files: BTreeMap<u32, Vec<SarcInnerFile>> = BTreeMap::new();
        for entry in fat_entries {
            let filename_hash = entry.filename_hash;
            let filename = if entry.filename_hash_count > 0 {
                let mut filename_buffer = Vec::new();
                buf.seek(SeekFrom::Start(&filename_table_start + (entry.filename_table_offset * 4) as u64))?;
                let filename_len = buf.read_until(0x0, &mut filename_buffer)?;
                Some(from_utf8(&filename_buffer[0..filename_len - 1]).unwrap().to_owned())
            } else {
                None
            };

            let start = &offset_to_data + entry.file_start as usize;
            let end = &offset_to_data + entry.file_end as usize;
            let data = Vec::from(&bytes[start..end]);

            let file = SarcInnerFile { filename, data };

            if let std::collections::btree_map::Entry::Vacant(e) = files.entry(filename_hash) {
                e.insert(vec![file]);
            } else {
                let hashed_files = files.get_mut(&filename_hash).unwrap();
                hashed_files.push(file);
            }
        }

        Ok(Self { path: path.to_owned(), files, multiplier, version })
    }

    /// Hash function used to hash filenames
    #[allow(unused)]
    fn calculate_hash(&self, filename: &str) -> u32 {
        filename.chars().fold(0, |hash, char| hash.wrapping_mul(self.multiplier.clone()) + (char as u32))
    }

    /// List all files in this SARC Archive, for research purposes
    #[allow(unused)]
    #[deprecated]
    pub(crate) fn list_files(&self) {
        info!("Listing Files in SARC Archive: {}", self.path);
        info!("Multiplier: {}", self.multiplier);

        for (filename_hash, files) in &self.files {
            info!("0x{:0>8X}: {:?}", filename_hash, files.iter().flat_map(|f| f.filename.clone()).collect::<Vec<_>>());
        }
    }
}

impl JackFile for Sarc {}

impl Pathed for Sarc {
    fn get_path(&self) -> &str {
        &self.path
    }
}

impl IntoBytes for Sarc {
    fn into_bytes(self) -> Box<[u8]> {
        // SFAT Section
        let mut sfat = Vec::with_capacity(0xC + (self.files.len() * 0x10));
        sfat.extend_from_slice(b"SFAT");
        sfat.extend_from_slice(&0xCu16.to_le_bytes());
        sfat.extend_from_slice(&(self.files.len() as u16).to_le_bytes());
        sfat.extend_from_slice(&self.multiplier.to_le_bytes());

        // SFNT Section
        let mut sfnt = Vec::new();
        sfnt.extend_from_slice(b"SFNT");
        sfnt.extend_from_slice(&0x8u32.to_le_bytes());

        // Data
        let mut data: Vec<u8> = Vec::new();

        /*
         * Iterate over all files, building out entries in the 3 tables as we go:
         * - FAT Entries in the SFAT Section store lookup info for the other 2 tables
         * - Filename Table in the SFNT Section optionally stores filenames (often these are empty, except ActorProfile.szs)
         * - Data Table stores the actual file data
         */
        let mut filename_table_idx = 0u32;
        let mut data_section_idx = 0u32;
        for (hash, bucket) in self.files {
            let bucket_len = bucket.len() as u32;

            for file in bucket {
                // Data Section

                /*
                 * Certain files have specific alignment requirements. These vary based on the file and I don't
                 * have a complete list. As a "catch-all", we're just aligning everything to 0x80 here, but this is
                 * certainly introducing a lot of unneeded padding.
                 * TODO: Have each file type know its own alignment requirement, so this can be removed.
                 */
                let file_data = align(file.data, 0x80, 0x0);
                let file_data_len = file_data.len();
                data.extend(file_data.into_iter());

                // Prep
                let file_data_start = data_section_idx;
                let file_data_end = file_data_start + file_data_len as u32;
                data_section_idx = file_data_end;
                let filename_attributes: u32;

                // SFNT Filename Table
                if let Some(filename) = file.filename {
                    let mut filename = format!("{}\0", filename); // add null terminator

                    // pad to 4 byte alignment
                    let padding_amt = 4 - (filename.len() % 4);
                    if padding_amt < 4 {
                        filename += &"\0".repeat(padding_amt);
                    }

                    sfnt.extend_from_slice(filename.as_bytes());

                    filename_attributes = (&bucket_len << 0x18) | (&filename_table_idx / 4);
                    filename_table_idx += filename.len() as u32;
                } else {
                    filename_attributes = 0;
                }

                // FAT Entry
                sfat.extend_from_slice(&hash.to_le_bytes());
                sfat.extend_from_slice(&filename_attributes.to_le_bytes());
                sfat.extend_from_slice(&file_data_start.to_le_bytes());
                sfat.extend_from_slice(&file_data_end.to_le_bytes());
            }
        }

        // SARC Header
        let sarc_header_len = 0x14;
        let mut offset_to_data = &sarc_header_len + sfat.len() + sfnt.len();
        let sfnt_padding = 0x80 - (&offset_to_data % 0x80); // align - TODO see above alignment comment, same thing
        offset_to_data += &sfnt_padding;

        let filesize = &offset_to_data + data.len();

        let mut szs = Vec::with_capacity(filesize.clone());
        szs.extend(b"SARC");
        szs.extend(&(sarc_header_len as u16).to_le_bytes());
        szs.extend(&0xFFFEu16.to_be_bytes());
        szs.extend(&(filesize as u32).to_le_bytes());
        szs.extend(&(offset_to_data as u32).to_le_bytes());
        szs.extend(&self.version.to_be_bytes());
        szs.extend(&0x0u16.to_le_bytes()); // padding

        szs.extend(sfat.into_iter());
        szs.extend(sfnt.into_iter());
        szs.extend(std::iter::repeat(0x0).take(sfnt_padding));
        szs.extend(data.into_iter());

        szs.into()
    }
}

///
#[derive(Debug, Clone)]
struct SarcInnerFile {
    filename: Option<String>,
    data: Vec<u8>,
}

///
fn align(data: Vec<u8>, alignment: usize, value: u8) -> Vec<u8> {
    let padding_amt = &alignment - (data.len() % &alignment);
    if padding_amt < alignment {
        let mut data = data;
        data.resize(data.len() + padding_amt, value);
        data
    } else {
        data
    }
}
