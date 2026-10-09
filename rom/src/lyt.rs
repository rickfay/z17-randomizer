use crate::{Error, File, Result, Sarc};
use bytey::*;

fn bytes_to_string(data: &[u8]) -> Result<String> {
    let pos = data.iter().position(|&c| c == 0);
    let data = if let Some(pos) = pos { &data[..pos] } else { data };
    str::from_utf8(data)
        .map(|s| s.to_string())
        .map_err(|_| Error::new("Error decoding UTF-8 string"))
}

fn string_to_bytes(string: &str, size: usize) -> Vec<u8> {
    let mut vec = string.to_owned().into_bytes();
    vec.resize(size, 0);
    vec
}

pub enum Section {
    Raw(Vec<u8>),
    Pane(Pane),
    Group(Group),
    Anim(AnimationInfo),
}

impl Section {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(match &data[..4] {
            b"pan1" | b"bnd1" | b"prt1" | b"wnd1" | b"pic1" | b"txt1" => Self::Pane(Pane::from_bytes(data)?),
            b"grp1" => Self::Group(Group::from_bytes(data)?),
            b"pai1" => Self::Anim(AnimationInfo::from_bytes(data)?),
            _ => Self::Raw(data.to_vec()),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Raw(data) => data.clone(),
            Self::Pane(pane) => pane.to_bytes(),
            Self::Group(group) => group.to_bytes(),
            Self::Anim(anim) => anim.to_bytes(),
        }
    }

    fn magic(&self) -> &[u8] {
        match self {
            Self::Raw(data) => &data[..4],
            Self::Pane(pane) => &pane.magic,
            Self::Group(_) => b"grp1",
            Self::Anim(_) => b"pai1",
        }
    }

    fn as_pane(&self) -> &Pane {
        match self {
            Self::Pane(pane) => pane,
            _ => { unreachable!(); }
        }
    }

    fn as_pane_mut(&mut self) -> &mut Pane {
        match self {
            Self::Pane(pane) => pane,
            _ => { unreachable!(); }
        }
    }
}

#[derive(Clone)]
pub struct Pane {
    pub magic: [u8; 4],
    pub size: u32,
    pub flags: u8,
    pub origin: u8,
    pub alpha: u8,
    pub part_scaling: u8,
    pub name: String,
    pub user_data_1: u32,
    pub user_data_2: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scl_x: f32,
    pub scl_y: f32,
    pub width: f32,
    pub height: f32,
    pub data: Vec<u8>,
}

impl Pane {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct PaneInfo: TryFromBytes<'_> [0x54] {
            [0] magic: [u8; 4],
            [4] size: u32,
            [8] flags: u8,
            [9] origin: u8,
            [0xa] alpha: u8,
            [0xb] part_scaling: u8,
            [0xc] name: [u8; 0x18],
            [0x24] user_data_1: u32,
            [0x28] user_data_2: u32,
            [0x2c] pos_x: f32,
            [0x30] pos_y: f32,
            [0x34] pos_z: f32,
            [0x38] rot_x: f32,
            [0x3c] rot_y: f32,
            [0x40] rot_z: f32,
            [0x44] scl_x: f32,
            [0x48] scl_y: f32,
            [0x4c] width: f32,
            [0x50] height: f32,
        }}

        let (info, rest) = PaneInfo::try_from_slice(data)?;
        Ok(Pane {
            magic: info.magic,
            size: info.size,
            flags: info.flags,
            origin: info.origin,
            alpha: info.alpha,
            part_scaling: info.part_scaling,
            name: bytes_to_string(&info.name)?,
            user_data_1: info.user_data_1,
            user_data_2: info.user_data_2,
            pos_x: info.pos_x,
            pos_y: info.pos_y,
            pos_z: info.pos_z,
            rot_x: info.rot_x,
            rot_y: info.rot_y,
            rot_z: info.rot_z,
            scl_x: info.scl_x,
            scl_y: info.scl_y,
            width: info.width,
            height: info.height,
            data: rest.to_vec(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(&self.magic);
        buf.extend(self.size.to_le_bytes());
        buf.push(self.flags);
        buf.push(self.origin);
        buf.push(self.alpha);
        buf.push(self.part_scaling);
        buf.extend(string_to_bytes(&self.name, 0x18));
        buf.extend(self.user_data_1.to_le_bytes());
        buf.extend(self.user_data_2.to_le_bytes());
        buf.extend(self.pos_x.to_le_bytes());
        buf.extend(self.pos_y.to_le_bytes());
        buf.extend(self.pos_z.to_le_bytes());
        buf.extend(self.rot_x.to_le_bytes());
        buf.extend(self.rot_y.to_le_bytes());
        buf.extend(self.rot_z.to_le_bytes());
        buf.extend(self.scl_x.to_le_bytes());
        buf.extend(self.scl_y.to_le_bytes());
        buf.extend(self.width.to_le_bytes());
        buf.extend(self.height.to_le_bytes());
        buf.extend(&self.data);
        buf
    }
}

pub struct Group {
    name: String,
    panes: Vec<String>,
}

impl Group {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0x24] {
            [0] magic: [u8; 4],
            [4] size: u32,
            [8] name: [u8; 0x18],
            [0x20] num_panes: u16,
        }}

        let (header, rest) = Header::try_from_slice(data)?;
        let panes = rest.chunks(0x18).map(|name| bytes_to_string(name)).collect::<Result<Vec<_>>>()?;
        Ok(Group {
            name: bytes_to_string(&header.name)?,
            panes,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(b"grp1");
        let size = 0x24 + 0x18 * self.panes.len();
        buf.extend((size as u32).to_le_bytes());
        buf.extend(string_to_bytes(&self.name, 0x18));
        buf.extend((self.panes.len() as u32).to_le_bytes());
        for pane_name in &self.panes {
            buf.extend(string_to_bytes(pane_name, 0x18));
        }
        buf
    }

    pub fn add_pane(&mut self, pane_name: &str) {
        self.panes.push(pane_name.to_string());
    }
}

#[derive(Clone)]
pub struct IntKeyframe {
    pub frame: f32,
    pub value: i16,
}

#[derive(Clone)]
pub struct FloatKeyframe {
    pub frame: f32,
    pub value: f32,
    pub blend: f32,
}

#[derive(Clone)]
pub enum Keyframes {
    Int(Vec<IntKeyframe>),
    Float(Vec<FloatKeyframe>),
}

impl Keyframes {
    fn data_type(&self) -> u8 {
        match self {
            Self::Int(_) => 1,
            Self::Float(_) => 2,
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Int(vec) => vec.len(),
            Self::Float(vec) => vec.len(),
        }
    }
}

#[derive(Debug)]
pub enum Target {
    TranslationX,
    TranslationY,
    TranslationZ,
}

impl Target {
    fn group(&self) -> &[u8; 4] {
        match self {
            Self::TranslationX | Self::TranslationY | Self::TranslationZ => b"FLPA",
        }
    }

    fn index(&self) -> u8 {
        match self {
            Self::TranslationX => 0,
            Self::TranslationY => 1,
            Self::TranslationZ => 2,
        }
    }
}

#[derive(Clone)]
pub struct AnimationTarget {
    flags: u8,
    target: u8,
    pub keyframes: Keyframes,
}

impl AnimationTarget {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0xC] {
            [0] flags: u8,
            [1] target: u8,
            [2] data_type: u8,
            [4] num_keyframes: u16,
        }}

        let (header, rest) = Header::try_from_slice(data)?;

        let keyframes = match header.data_type {
            1 => {
                let mut keyframes = Vec::new();
                let mut data = rest;
                typedef! { struct Keyframe: TryFromBytes<'_> [8] {
                    [0] frame: f32,
                    [4] value: i16,
                }}
                for _ in 0 .. header.num_keyframes as usize {
                    let (keyframe, rest) = Keyframe::try_from_slice(data)?;
                    data = rest;
                    keyframes.push(IntKeyframe {
                        frame: keyframe.frame,
                        value: keyframe.value,
                    });
                }
                Keyframes::Int(keyframes)
            },
            2 => {
                let mut keyframes = Vec::new();
                let mut data = rest;
                typedef! { struct Keyframe: TryFromBytes<'_> [0xC] {
                    [0] frame: f32,
                    [4] value: f32,
                    [8] blend: f32,
                }}
                for _ in 0 .. header.num_keyframes as usize {
                    let (keyframe, rest) = Keyframe::try_from_slice(data)?;
                    data = rest;
                    keyframes.push(FloatKeyframe {
                        frame: keyframe.frame,
                        value: keyframe.value,
                        blend: keyframe.blend,
                    });
                }
                Keyframes::Float(keyframes)
            },
            _ => { return Err(Error::new("Invalid animation target data type")); }
        };

        Ok(AnimationTarget {
            flags: header.flags,
            target: header.target,
            keyframes,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.flags);
        buf.push(self.target);
        buf.push(self.keyframes.data_type());
        buf.push(0);
        buf.extend((self.keyframes.len() as u32).to_le_bytes());
        buf.extend(0xCu32.to_le_bytes());
        match &self.keyframes {
            Keyframes::Int(keyframes) => {
                for keyframe in keyframes {
                    buf.extend(keyframe.frame.to_le_bytes());
                    buf.extend(keyframe.value.to_le_bytes());
                    buf.extend([0, 0]);
                }
            },
            Keyframes::Float(keyframes) => {
                for keyframe in keyframes {
                    buf.extend(keyframe.frame.to_le_bytes());
                    buf.extend(keyframe.value.to_le_bytes());
                    buf.extend(keyframe.blend.to_le_bytes());
                }
            },
        }
        buf
    }
}

#[derive(Clone)]
pub struct AnimationTargetGroup {
    magic: [u8; 4],
    targets: Vec<AnimationTarget>,
}

impl AnimationTargetGroup {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [8] {
            [0] magic: [u8; 4],
            [4] num_targets: u32,
        }}

        let (header, rest) = Header::try_from_slice(data)?;
        let mut targets = Vec::new();

        for i in 0 .. header.num_targets as usize {
            let (offset, _) = u32::try_from_slice(&rest[4 * i ..])?;
            targets.push(AnimationTarget::from_bytes(&data[offset as usize ..])?);
        }

        Ok(AnimationTargetGroup {
            magic: header.magic,
            targets,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(&self.magic);
        buf.extend((self.targets.len() as u32).to_le_bytes());
        for _ in 0..self.targets.len() {
            buf.extend(0u32.to_le_bytes());
        }
        for i in 0..self.targets.len() {
            let offset = buf.len() as u32;
            buf[8 + 4 * i .. 8 + 4 * i + 4].copy_from_slice(&offset.to_le_bytes());
            buf.extend(self.targets[i].to_bytes());
        }
        buf
    }
}

#[derive(Clone)]
pub struct AnimationObject {
    pub name: String,
    kind: u8,
    target_groups: Vec<AnimationTargetGroup>,
}

impl AnimationObject {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0x20] {
            [0] name: [u8; 0x1c],
            [0x1c] num_groups: u8,
            [0x1d] kind: u8,
        }}

        let (header, rest) = Header::try_from_slice(data)?;
        let mut target_groups = Vec::new();

        for i in 0 .. header.num_groups as usize {
            let (offset, _) = u32::try_from_slice(&rest[4 * i ..])?;
            target_groups.push(AnimationTargetGroup::from_bytes(&data[offset as usize ..])?);
        }

        Ok(AnimationObject {
            name: bytes_to_string(&header.name)?,
            kind: header.kind,
            target_groups,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(string_to_bytes(&self.name, 0x1c));
        buf.push(self.target_groups.len() as u8);
        buf.push(self.kind);
        buf.extend([0, 0]);
        for _ in 0..self.target_groups.len() {
            buf.extend(0u32.to_le_bytes());
        }
        for i in 0..self.target_groups.len() {
            let offset = buf.len() as u32;
            buf[0x20 + 4 * i .. 0x20 + 4 * i + 4].copy_from_slice(&offset.to_le_bytes());
            buf.extend(self.target_groups[i].to_bytes());
        }
        buf
    }

    pub fn get_target_mut(&mut self, target: Target) -> Result<&mut AnimationTarget> {
        for group in &mut self.target_groups {
            if group.magic == *target.group() {
                for anim_target in &mut group.targets {
                    if anim_target.target == target.index() {
                        return Ok(anim_target);
                    }
                }
            }
        }
        return Err(Error::new(format!("No animation found for target {:?}", target)));
    }
}

pub struct AnimationInfo {
    frames: u16,
    flags: u16,
    textures: Vec<String>,
    objects: Vec<AnimationObject>,
}

impl AnimationInfo {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0x14] {
            [0] magic: [u8; 4],
            [4] size: u32,
            [8] frames: u16,
            [0xa] flags: u16,
            [0xc] num_textures: u16,
            [0xe] num_objects: u16,
            [0x10] object_table_offset: u32,
        }}

        let (header, _) = Header::try_from_slice(data)?;
        let mut textures = Vec::new();
        let mut objects = Vec::new();

        for i in 0 .. header.num_textures as usize {
            let (offset, _) = u32::try_from_slice(&data[0x14 + 4 * i ..])?;
            textures.push(bytes_to_string(&data[0x14 + offset as usize ..])?);
        }

        for i in 0 .. header.num_objects as usize {
            let (offset, _) = u32::try_from_slice(&data[header.object_table_offset as usize + 4 * i ..])?;
            objects.push(AnimationObject::from_bytes(&data[offset as usize ..])?);
        }

        Ok(AnimationInfo {
            frames: header.frames,
            flags: header.flags,
            textures,
            objects,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(b"pai1");
        buf.extend(0u32.to_le_bytes());
        buf.extend(self.frames.to_le_bytes());
        buf.extend(self.flags.to_le_bytes());
        buf.extend((self.textures.len() as u16).to_le_bytes());
        buf.extend((self.objects.len() as u16).to_le_bytes());
        buf.extend((0x14u32 + self.textures.len() as u32 * 4).to_le_bytes());
        for _ in 0..self.textures.len() {
            buf.extend(0u32.to_le_bytes());
        }
        for _ in 0..self.objects.len() {
            buf.extend(0u32.to_le_bytes());
        }
        for i in 0..self.textures.len() {
            let offset = buf.len() as u32;
            buf[0x14 + 4 * i .. 0x14 + 4 * i + 4].copy_from_slice(&offset.to_le_bytes());
            buf.extend(self.textures[i].clone().into_bytes());
            buf.push(0);
        }
        for i in 0..self.objects.len() {
            let offset = buf.len() as u32;
            let offset_start = 0x14 + 4 * self.textures.len() + 4 * i;
            buf[offset_start .. offset_start + 4].copy_from_slice(&offset.to_le_bytes());
            buf.extend(self.objects[i].to_bytes());
        }
        buf
    }
}

pub struct Layout {
    sections: Vec<Section>,
}

impl Layout {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0x14] {
            #b"FLYT",
            [4] bom: u16 where bom == 0xfeff,
            [6] header_size: u16 where header_size == 0x14,
            [8] version: u32 where version == 0x03000000,
            [0xc] file_size: u32,
            [0x10] section_count: u32,
        }}

        let (header, mut rest) = Header::try_from_slice(data)?;
        let mut sections = Vec::new();

        for _ in 0..header.section_count {
            let (size, _) = u32::try_from_slice(&rest[4..])?;
            sections.push(Section::from_bytes(&rest[0..(size as usize)])?);
            rest = &rest[(size as usize)..];
        }

        Ok(Layout { sections })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(b"FLYT");
        buf.extend([0xff, 0xfe, 0x14, 0x00, 0x00, 0x00, 0x00, 0x03]);
        buf.extend([0x00, 0x00, 0x00, 0x00]);
        buf.extend((self.sections.len() as u32).to_le_bytes());

        for section in &self.sections {
            buf.extend(&section.to_bytes());
        }

        let size = buf.len() as u32;
        buf[0xc..0x10].copy_from_slice(&size.to_le_bytes());

        buf
    }

    pub fn get_pane_index(&self, name: &str) -> Option<usize> {
        for index in 0..self.sections.len() {
            if let Section::Pane(pane) = &self.sections[index] {
                if pane.name == name {
                    return Some(index);
                }
            }
        }
        return None;
    }

    pub fn get_pane(&self, name: &str) -> Result<&Pane> {
        self.get_pane_index(name)
            .map(|index| self.sections[index].as_pane())
            .ok_or(Error::new(format!("No pane named {}", name)))
    }

    pub fn get_pane_mut(&mut self, name: &str) -> Result<&mut Pane> {
        self.get_pane_index(name)
            .map(|index| self.sections[index].as_pane_mut())
            .ok_or(Error::new(format!("No pane named {}", name)))
    }

    pub fn add_child(&mut self, parent_name: &str, pane: Pane) -> Result<()> {
        let mut index = self.get_pane_index(parent_name)
            .ok_or(Error::new(format!("No pane named {}", parent_name)))?;
        index += 1;
        let mut depth = 0;
        while index < self.sections.len() {
            if self.sections[index].magic() == b"pas1" {
                depth += 1;
            } else if self.sections[index].magic() == b"pae1" {
                depth -= 1;
            }
            if depth == 0 {
                self.sections.insert(index, Section::Pane(pane));
                return Ok(());
            }
            index += 1;
        }
        Err(Error::new("End of file reached before pane ended"))
    }

    pub fn get_group_mut(&mut self, name: &str) -> Result<&mut Group> {
        for section in &mut self.sections {
            if let Section::Group(group) = section {
                if group.name == name {
                    return Ok(group);
                }
            }
        }
        Err(Error::new(format!("No group named {}", name)))
    }
}

pub struct Animation {
    sections: Vec<Section>,
}

impl Animation {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0x14] {
            #b"FLAN",
            [4] bom: u16 where bom == 0xfeff,
            [6] header_size: u16 where header_size == 0x14,
            [8] version: u32 where version == 0x03000000,
            [0xc] file_size: u32,
            [0x10] section_count: u32,
        }}

        let (header, mut rest) = Header::try_from_slice(data)?;
        let mut sections = Vec::new();

        for _ in 0..header.section_count {
            let (size, _) = u32::try_from_slice(&rest[4..])?;
            sections.push(Section::from_bytes(&rest[0..(size as usize)])?);
            rest = &rest[(size as usize)..];
        }

        Ok(Animation { sections })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(b"FLAN");
        buf.extend([0xff, 0xfe, 0x14, 0x00, 0x00, 0x00, 0x00, 0x03]);
        buf.extend([0x00, 0x00, 0x00, 0x00]);
        buf.extend((self.sections.len() as u32).to_le_bytes());

        for section in &self.sections {
            buf.extend(&section.to_bytes());
        }

        let size = buf.len() as u32;
        buf[0xc..0x10].copy_from_slice(&size.to_le_bytes());

        buf
    }

    pub fn get_info(&self) -> Result<&AnimationInfo> {
        for section in &self.sections {
            if let Section::Anim(info) = section {
                return Ok(info);
            }
        }
        return Err(Error::new("Animation does not contain a pai section"));
    }

    pub fn get_info_mut(&mut self) -> Result<&mut AnimationInfo> {
        for section in &mut self.sections {
            if let Section::Anim(info) = section {
                return Ok(info);
            }
        }
        return Err(Error::new("Animation does not contain a pai section"));
    }

    pub fn get_object(&self, name: &str) -> Result<&AnimationObject> {
        let info = self.get_info()?;
        for object in &info.objects {
            if object.name == name {
                return Ok(object);
            }
        }
        return Err(Error::new(format!("No animation object named {}", name)));
    }

    pub fn get_object_mut(&mut self, name: &str) -> Result<&mut AnimationObject> {
        let info = self.get_info_mut()?;
        for object in &mut info.objects {
            if object.name == name {
                return Ok(object);
            }
        }
        return Err(Error::new(format!("No animation object named {}", name)));
    }

    pub fn add_object(&mut self, object: AnimationObject) -> Result<()> {
        let info = self.get_info_mut()?;
        info.objects.push(object);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Lyt {
    pub archive: File<Sarc>,
}

impl Lyt {
    pub fn new(archive: File<Sarc>) -> Self {
        Self { archive }
    }

    pub fn get_layout(&mut self, name: &str) -> Result<File<Layout>> {
        let path = format!("blyt/{}.bflyt", name);
        self.archive.get_mut().open(path)?.try_map(|data| Layout::from_bytes(&data))
    }

    pub fn update_layout(&mut self, file: File<Layout>) -> Result<()> {
        self.archive.get_mut().update(file.map(|layout| layout.to_bytes()).into_bytes())
    }

    pub fn get_animation(&mut self, name: &str) -> Result<File<Animation>> {
        let path = format!("anim/{}.bflan", name);
        self.archive.get_mut().open(path)?.try_map(|data| Animation::from_bytes(&data))
    }

    pub fn update_animation(&mut self, file: File<Animation>) -> Result<()> {
        self.archive.get_mut().update(file.map(|layout| layout.to_bytes()).into_bytes())
    }
}
