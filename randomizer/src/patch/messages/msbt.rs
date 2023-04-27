use {
    crate::{Patcher, Result},
    albw::{course::Id, File},
    byteorder::{ByteOrder, LittleEndian},
    std::str::from_utf8,
};

/// MSBT File
///
/// These files contain text strings that are looked up by a label.
///
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/MSBT-File-Format
pub(crate) struct MsbtFile {
    filename: String,
    course: Id,
    lbl1: Lbl1Block,
    #[allow(unused)]
    atr1: Atr1Block,
    txt2: Txt2Block,
}

impl MsbtFile {
    /// Looks up and returns the message specified by `label`
    #[allow(unused)]
    pub(crate) fn get(&self, label: &str) -> Option<&String> {
        return if let Some(item_index) = self.get_item_index(label) {
            self.txt2.messages.get(item_index)
        } else {
            None
        };
    }

    /// Looks up the message specified by `label` and sets it to `value`
    /// Appends a null terminator `\0` to the end of value.
    pub(crate) fn set(&mut self, label: &str, value: &str) {
        if let Some(item_index) = self.get_item_index(label) {
            let _ = std::mem::replace(&mut self.txt2.messages[item_index], format!("{}\0", value));
        }
    }

    /// Add a new `message` to the [`MsbtFile`] with the corresponding `label`
    #[allow(unused)]
    pub(crate) fn add(&self, label: &str, message: &str) {
        unimplemented!();
    }

    fn get_item_index(&self, key: &str) -> Option<usize> {
        let hash = calc_hash(String::from(key), self.lbl1.num_slots) as usize;
        let hash_table_slot = self.lbl1.hash_table.get(hash).unwrap();

        return if let Some(label) = hash_table_slot.labels.iter().find(|&label| label.label.eq(key))
        {
            Some(label.item_index as usize)
        } else {
            None
        };
    }

    #[allow(unused)]
    pub(crate) fn debug(&self) {
        let mut labels =
            self.lbl1.hash_table.iter().flat_map(|slot| slot.labels.iter()).collect::<Vec<_>>();
        labels.sort_by(|this, that| this.item_index.cmp(&that.item_index));

        for label in &labels {
            println!(
                "\nIndex: {}\nLabel: \"{}\"\n\"{}\"\n\n",
                label.item_index,
                label.label,
                self.txt2.messages.get(label.item_index as usize).unwrap()
            );
        }
    }

    /// Builds a valid `.msbt` file and dumps it as a [`File<Vec<u8>>`]
    pub(crate) fn dump(&self) -> (Id, File<Vec<u8>>) {
        // LBL1 BLOCK
        let mut hash_table_buffer = Vec::new();
        let mut labels_buffer = Vec::new();
        for entry in &self.lbl1.hash_table {
            hash_table_buffer.extend_from_slice(&entry.number_of_labels.to_le_bytes());
            hash_table_buffer.extend_from_slice(&entry.offset_to_labels.to_le_bytes());

            for label in &entry.labels {
                labels_buffer.push(label.label.len() as u8);
                labels_buffer.extend_from_slice(label.label.as_bytes());
                labels_buffer.extend_from_slice(&label.item_index.to_le_bytes());
            }
        }

        let lbl1_content_size = 0x4 + hash_table_buffer.len() + labels_buffer.len();
        let lbl1_block_size = (lbl1_content_size & 0xFFFFFFF0) + 0x20;

        let mut lbl1_block = Vec::with_capacity(lbl1_block_size);
        lbl1_block.extend_from_slice(b"LBL1");
        lbl1_block.extend_from_slice(&(lbl1_content_size as u32).to_le_bytes());
        lbl1_block.extend(std::iter::repeat(0x0u8).take(8));
        lbl1_block.extend(&self.lbl1.num_slots.to_le_bytes());
        lbl1_block.extend_from_slice(&hash_table_buffer);
        lbl1_block.extend_from_slice(&labels_buffer);
        lbl1_block.extend(std::iter::repeat(0xAB).take(lbl1_block_size - lbl1_block.len()));

        // TXT2 BLOCK
        let mut msg_offset_buffer: Vec<u8> = Vec::new();
        let mut msgs_buffer: Vec<u8> = Vec::new();

        let num_msgs = self.txt2.messages.len();
        let msgs_init_offset = ((num_msgs * 0x4) + 0x4) as u32;

        let mut prev_msg_len = 0;
        for message in &self.txt2.messages {
            msg_offset_buffer.extend_from_slice(&(msgs_init_offset + prev_msg_len).to_le_bytes());
            let msg =
                message.encode_utf16().flat_map(|thing| thing.to_le_bytes()).collect::<Vec<u8>>();
            msgs_buffer.extend_from_slice(&msg);
            prev_msg_len += msg.len() as u32;
        }

        let txt2_content_size = 0x4 + msg_offset_buffer.len() + msgs_buffer.len();
        let txt2_block_size = (txt2_content_size & 0xFFFFFFF0) + 0x20;

        let mut txt2_block: Vec<u8> = Vec::with_capacity(txt2_block_size);
        txt2_block.extend_from_slice(b"TXT2");
        txt2_block.extend_from_slice(&(txt2_content_size as u32).to_le_bytes());
        txt2_block.extend(std::iter::repeat(0x0u8).take(8));
        txt2_block.extend_from_slice(&(num_msgs as u32).to_le_bytes());
        txt2_block.extend_from_slice(&msg_offset_buffer);
        txt2_block.extend_from_slice(&msgs_buffer);
        txt2_block.extend(std::iter::repeat(0xAB).take(txt2_block_size - txt2_block.len()));

        // ATR1 BLOCK
        let mut atr1_block = Vec::with_capacity(0x20);
        atr1_block.extend_from_slice(b"ATR1");
        atr1_block.extend_from_slice(&8u32.to_le_bytes());
        atr1_block.extend(std::iter::repeat(0x0).take(8));
        atr1_block.extend_from_slice(&(num_msgs as u64).to_le_bytes());
        atr1_block.extend(std::iter::repeat(0xAB).take(8));

        // MSBT FILE HEADER
        let mut file_header = Vec::with_capacity(0x20);
        let file_size = 0x20 + lbl1_block.len() + atr1_block.len() + txt2_block.len();

        file_header.extend_from_slice(b"MsgStdBn");
        file_header.extend_from_slice(&[0xFF, 0xFE]);
        file_header.extend(std::iter::repeat(0x0).take(2)); // padding
        file_header.push(0x1); // message encoding (1 = UTF-16)
        file_header.push(0x3); // version number
        file_header.extend_from_slice(&0x3u16.to_le_bytes()); // number of blocks
        file_header.extend(std::iter::repeat(0x0).take(2)); // padding
        file_header.extend_from_slice(&(file_size as u32).to_le_bytes());
        file_header.extend(std::iter::repeat(0x0).take(10)); // padding

        let mut msbt_file = Vec::with_capacity(file_size);
        msbt_file.extend_from_slice(&file_header);
        msbt_file.extend_from_slice(&lbl1_block);
        msbt_file.extend_from_slice(&atr1_block);
        msbt_file.extend_from_slice(&txt2_block);

        (self.course, File::new(self.filename.clone(), Vec::from(msbt_file)))
    }
}

/// MSBT File Header
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/LMS-File-Format#file-header
#[allow(unused)]
#[derive(Default)]
struct MsbtFileHeader {
    /// Msbt Files have the magic number: `MsgStdBn`
    magic_number: u64,
    /// * `0xFEFF` = Big Endian
    /// * `0xFFFE` = Little Endian
    endianness: u16,
    /// ALBW MSBT Files all use UTF-16
    /// * `0x0` = UTF-8
    /// * `0x1` = UTF-16
    /// * `0x2` = UTF-32
    msg_encoding: u8,
    /// Always `0x3`
    version_num: u8,
    /// This seems to always(?) be 3 for ALBW, for the `LBL1`, `ATR1`, and `TXT2` blocks.
    num_blocks: u16,
    /// The size of the entire file, *including* this header block.
    file_size: u32,
}

/// Header for blocks found in MSBT / MSBP files
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/LMS-File-Format#block-header
#[derive(Default)]
struct BlockHeader {
    /// Magic number for the block
    block_type: u32,
    /// Size of the block, *NOT* including this header block.
    block_size: u32,
}

/// Label Block
/// Contains a hash table of labels used to lookup messages from the other blocks.
///
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/MSBT-File-Format#lbl1-block
#[derive(Default)]
struct Lbl1Block {
    /// Uses the magic number `LBL1`
    header: BlockHeader,
    /// The number of slots in the Hash Table.
    /// This is always:
    /// * 101 in MSBT Files
    /// * 29 in MSBP Files
    num_slots: u32,
    /// Labels Hash Table
    /// Use the [`calc_hash`] function to hash a label (the key) to get the index.
    hash_table: Vec<HashTableSlot>,
}

/// Hash Table Slot
///
/// Represents a bucket in the hash table.
/// Ultimately this is used to lookup the message index in the [`Txt2Block`].
///
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/LMS-File-Format#hash-tables
#[derive(Default)]
struct HashTableSlot {
    /// Number of labels in this bucket.
    number_of_labels: u32,
    /// Offset to the first [`Label`] in this bucket within the [`Lbl1Block`].
    offset_to_labels: u32,
    /// The actual [`Label`]s referenced by this bucket.
    labels: Vec<Label>,
}

/// Labels used to lookup messages from the [`Txt2Block`].
#[derive(Default)]
struct Label {
    /// An arbitrary label acting as a key to lookup a message.
    ///
    /// To deal with collisions arising from the [`Lbl1Block`]'s hash table, the lookup label key
    /// must be compared against this label to determine which entry in the bucket is correct.
    label: String,
    /// The message index within the [`Txt2Block`].
    item_index: u32,
}

/// Attributes Block
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/MSBT-File-Format#atr1-block
#[derive(Default)]
struct Atr1Block {
    /// Uses the magic number `ATR1`
    header: BlockHeader,
    /// Count of the number of messages in this file.
    ///
    /// This really *should* be a [`Vec`] of different attributes used by this file, but we only
    /// ever seem to need this one so the data structure has been simplified.
    count: u64, // there only ever seems to be this one attribute, not bothering with a Vec
}

/// Text Block
///
/// Stores the actual text messages as UTF-16 strings.
///
/// Reference: https://github.com/Kinnay/Nintendo-File-Formats/wiki/MSBT-File-Format#txt2-block
#[derive(Default)]
struct Txt2Block {
    /// Uses the magic number `TXT2`
    header: BlockHeader,
    /// The number of messages in this file. Should match the `count` in the [`Atr1Block`].
    num_messages: u32,
    /// The actual text messages.
    messages: Vec<String>,
}

/// Load MSBT File
pub(crate) fn load_msbt(patcher: &mut Patcher, course: Id, file: &str) -> Result<MsbtFile> {
    let filename = format!("US_English/{}.msbt", file);
    let mut file =
        patcher.language(course.clone()).unwrap().flow().extract(filename.as_str()).unwrap();

    let raw = file.get_mut();

    // FILE HEADER
    // let mut file_header = MsbtFileHeader::default();
    // file_header.magic_number = LittleEndian::read_u64(&raw[0x0..]); // MsgStdBn
    // file_header.endianness = LittleEndian::read_u16(&raw[0x8..]); // FFFE = Little Endian
    // file_header.msg_encoding = raw[0xC]; // 1 (indicates we're using UTF-16)
    // file_header.version_num = raw[0xD]; // Always 3
    // file_header.num_blocks = LittleEndian::read_u16(&raw[0xE..]);
    // file_header.file_size = LittleEndian::read_u32(&raw[0x12..]);

    // LBL1 HEADER BLOCK
    let mut lbl1 = Lbl1Block::default();
    lbl1.header.block_type = LittleEndian::read_u32(&raw[0x20..]); // LBL1
    lbl1.header.block_size = LittleEndian::read_u32(&raw[0x24..]);
    lbl1.num_slots = LittleEndian::read_u32(&raw[0x30..]); // Should always be 101 for MSBT, 29 in MSBP

    // LBL1 Hash Table
    lbl1.hash_table = Vec::new();
    for i in 0..lbl1.num_slots {
        let number_of_labels = LittleEndian::read_u32(&raw[0x34 + (i as usize * 8)..]);
        let offset_to_labels = LittleEndian::read_u32(&raw[0x38 + (i as usize * 8)..]);

        let mut labels = Vec::with_capacity(number_of_labels as usize);
        let mut prev_label_size = 0;
        for _ in 0..number_of_labels {
            let label_length = raw[0x30 + prev_label_size + offset_to_labels as usize];
            let label_start_idx = 0x31 + prev_label_size + offset_to_labels as usize;
            let label_end_idx = label_start_idx + label_length as usize;

            let label = from_utf8(&raw[label_start_idx..label_end_idx]).unwrap().to_owned();
            let item_index = LittleEndian::read_u32(&raw[label_end_idx..]);

            labels.push(Label { label, item_index });

            prev_label_size += (label_length + 0x5) as usize;
        }

        // Hash Table Slots
        lbl1.hash_table.push(HashTableSlot { number_of_labels, offset_to_labels, labels });
    }

    // ATR1 BLOCK

    let atr1_idx = (((lbl1.header.block_size + 0x30) & 0xFFFFFFF0) + 0x10) as usize;

    let mut atr1 = Atr1Block::default();
    atr1.header.block_type = LittleEndian::read_u32(&raw[atr1_idx..]); // ATR1
    atr1.header.block_size = LittleEndian::read_u32(&raw[atr1_idx + 0x4..]); // Should be 8
    atr1.count = LittleEndian::read_u64(&raw[atr1_idx + 0x10..]); // Should match label count

    // TXT2 BLOCK
    let txt2_block = atr1_idx + 0x20;

    let mut txt2 = Txt2Block::default();
    txt2.header.block_type = LittleEndian::read_u32(&raw[txt2_block..]); // Number of messages
    txt2.header.block_size = LittleEndian::read_u32(&raw[txt2_block + 0x4..]);

    txt2.num_messages = LittleEndian::read_u32(&raw[txt2_block + 0x10..]); // Number of messages

    let msg_idx = txt2_block + 0x10;
    let mut msg_offsets = Vec::new();
    for i in 0..txt2.num_messages {
        msg_offsets.push(LittleEndian::read_u32(&raw[msg_idx + 0x4 + (i * 4) as usize..]));
    }

    txt2.messages = Vec::new();
    for i in 0..txt2.num_messages {
        let i: usize = i as usize;

        let start_idx = msg_idx + msg_offsets[i] as usize;
        let end_idx = if i < (txt2.num_messages - 1) as usize {
            msg_idx + msg_offsets[i + 1] as usize
        } else {
            msg_idx + txt2.header.block_size as usize
        };

        txt2.messages.push(unsafe {
            String::from_utf16(raw[start_idx..end_idx].align_to::<u16>().1).unwrap()
        });
    }

    Ok(MsbtFile { filename, course, lbl1, atr1, txt2 })
}

/// Standard Hash Function used by MSBT/MSBP files for lookups
/// https://github.com/Kinnay/Nintendo-File-Formats/wiki/LMS-File-Format#hash-table-slot
fn calc_hash(label: String, num_slots: u32) -> u32 {
    let mut hash: u32 = 0;
    for char in label.chars() {
        hash = (u128::from(hash) * 0x492u128) as u32 + (char as u32);
    }
    ((hash & 0xFFFFFFFF) % num_slots) as u32
}
