pub mod msbf;
pub mod msbt;

#[derive(Default)]
struct BlockHeader {
    block_type: u32,
    block_size: u32,
}

#[derive(Default)]
struct Fen1HashTable {
    header: BlockHeader,
    num_slots: u32,
    hash_table: Vec<Fen1HashTableSlot>,
}

#[derive(Default)]
struct Fen1HashTableSlot {
    number_of_labels: u32,
    offset_to_labels: u32,
}

#[derive(Default)]
struct Label {
    label: String,
    item_index: u32,
}

/// Standard Hash Function used by LMS files for lookups
/// https://github.com/Kinnay/Nintendo-File-Formats/wiki/LMS-File-Format#hash-table-slot
pub(crate) fn calc_lms_hash(label: String, num_slots: u32) -> u32 {
    label.chars().fold(0u32, |hash, char| hash.wrapping_mul(0x492) + (char as u32)) % num_slots
}
