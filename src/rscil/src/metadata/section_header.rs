use super::characteristics::SectionCharacteristics;

/// # II.25.3 Section headers 
///
/// Immediately following the optional header is the Section Table, which contains a number of section 
/// headers. This positioning is required because the file header does not contain a direct pointer to the 
/// section table; the location of the section table is determined by calculating the location of the first byte 
/// after the headers. 
/// 
/// Each section header has the following format, for a total of `40` bytes per entry: 
///
/// | Offset | Size | Field                 | Description |
/// | ------ | ---- | --------------------- | ----------- |
/// | 0      | 8    | Name                  | An 8-byte, null-padded ASCII string. There is no terminating null if the string is exactly eight characters long. |
/// | 8      | 4    | VirtualSize           | Total size of the section in bytes. If this value is greater than SizeOfRawData, the section is zero-padded. |
/// | 12     | 4    | VirtualAddress        | For executable images this is the address of the first byte of the section, when loaded into memory, relative to the image base. |
/// | 16     | 4    | SizeOfRawData         | Size of the initialized data on disk in bytes, shall be a multiple of FileAlignment from the PE header. If this is less than VirtualSize the remainder of the section is zero filled. Because this field is rounded while the VirtualSize field is not it is possible for this to be greater than VirtualSize as well. When a section contains only uninitialized data, this field should be 0. |
/// | 20     | 4    | PointerToRawData      | Offset of section’s first page within the PE file. This shall be a multiple of FileAlignment from the optional header. When a section contains only uninitialized data, this field should be 0. |
/// | 24     | 4    | PointerToRelocations  | Should be 0 (§II.24.1). |
/// | 28     | 4    | PointerToLinenumbers  | Should be 0 (§II.24.1). |
/// | 32     | 2    | NumberOfRelocations   | Should be 0 (§II.24.1). |
/// | 34     | 2    | NumberOfLinenumbers   | Should be 0 (§II.24.1). |
/// | 36     | 4    | Characteristics       | Flags describing section's characteristics; see below. |
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: SectionCharacteristics,
}

impl SectionHeader {
    pub fn from(slice: &[u8; 40]) -> SectionHeader {
        SectionHeader {
            name: slice[0..8].try_into().unwrap(),
            virtual_size: u32::from_le_bytes(slice[8..12].try_into().unwrap()),
            virtual_address: u32::from_le_bytes(slice[12..16].try_into().unwrap()),
            size_of_raw_data: u32::from_le_bytes(slice[16..20].try_into().unwrap()),
            pointer_to_raw_data: u32::from_le_bytes(slice[20..24].try_into().unwrap()),
            pointer_to_relocations: u32::from_le_bytes(slice[24..28].try_into().unwrap()),
            pointer_to_linenumbers: u32::from_le_bytes(slice[28..32].try_into().unwrap()),
            number_of_relocations: u16::from_le_bytes(slice[32..34].try_into().unwrap()),
            number_of_linenumbers: u16::from_le_bytes(slice[34..36].try_into().unwrap()),
            characteristics: SectionCharacteristics::new(u32::from_le_bytes(slice[36..40].try_into().unwrap())),
        }
    }
}
