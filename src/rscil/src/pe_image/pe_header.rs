use super::characteristics::FileCharacteristics;

/// # II.25.2.2 PE file header 
/// 
/// Immediately after the PE signature is the PE File header consisting of the following: 
/// 
/// | Offset | Size | Field                   | Description |
/// | ------ | ---- | ----------------------- | ----------- |
/// | 0x00   | 2    | Machine                 | Always `0x14c`. |
/// | 0x02   | 2    | Number of Sections      | Number of sections; indicates size of the Section Table, which immediately follows the headers. |
/// | 0x04   | 4    | Time/Date Stamp         | Time and date the file was created in seconds since `January 1st 1970 00:00:00` or `0`. |
/// | 0x08   | 4    | Pointer to Symbol Table | Always `0` (§II.24.1). |
/// | 0x0c   | 4    | Number of Symbols       | Always `0` (§II.24.1). |
/// | 0x10   | 2    | Optional Header Size    | Size of the optional header, the format is described below. |
/// | 0x12   | 2    | Characteristics         | Flags indicating attributes of the file, see §II.25.2.2.1. |
pub struct PeHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub optional_header_size: u16,
    pub characteristics: FileCharacteristics,
}

impl PeHeader {
    pub fn from(slice: &[u8; 20]) -> PeHeader {
        PeHeader {
            machine: u16::from_le_bytes(slice[0..2].try_into().unwrap()),
            number_of_sections: u16::from_le_bytes(slice[2..4].try_into().unwrap()),
            time_date_stamp: u32::from_le_bytes(slice[4..8].try_into().unwrap()),
            pointer_to_symbol_table: u32::from_le_bytes(slice[8..12].try_into().unwrap()),
            number_of_symbols: u32::from_le_bytes(slice[12..16].try_into().unwrap()),
            optional_header_size: u16::from_le_bytes(slice[16..18].try_into().unwrap()),
            characteristics: FileCharacteristics::new(u16::from_le_bytes(slice[18..20].try_into().unwrap())),
        }
    }
}