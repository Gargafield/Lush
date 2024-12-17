
/// # II.25.2.3.1 PE header standard fields 
/// 
/// These fields are required for all PE files and contain the following information: 
/// 
/// | Offset | Size | Field         | Description |
/// | ------ | ---- | ------------- | ----------- |
/// | 0      | 2    | Magic         | Always `0x10B`. |
/// | 2      | 1    | LMajor        | Always `6` (§II.24.1). |
/// | 3      | 1    | LMinor        | Always `0` (§II.24.1). |
/// | 4      | 4    | Code          | Size Size of the code (text) section, or the sum of all code sections if there are multiple sections. |
/// | 8      | 4    | Initialized   | Data Size Size of the initialized data section, or the sum of all such sections if there are multiple data sections. |
/// | 12     | 4    | Uninitialized | Data Size Size of the uninitialized data section, or the sum of all such sections if there are multiple uninitialized data sections. |
/// | 16     | 4    | Entry Point   | RVA RVA of entry point, needs to point to bytes `0xFF` `0x25` followed by the RVA in a section marked execute/read for EXEs or 0 for DLLs |
/// | 20     | 4    | Base Of Code  | RVA of the code section. (This is a hint to the loader.) |
/// | 24     | 4    | Base Of Data  | RVA of the data section. (This is a hint to the loader.) |
/// 
/// The entry point RVA shall always be either the `x86` entry point stub or be `0`. On non-CLI aware 
/// platforms, this stub will call the entry point API of `mscoree` (`_CorExeMain` or `_CorDllMain`). The 
/// `mscoree` entry point will use the module handle to load the metadata from the image, and invoke the 
/// entry point specified in vthe CLI header.
pub struct StandardFields {
    magic: u16,
    l_major: u8,
    l_minor: u8,
    code_size: u32,
    initialized_data_size: u32,
    uninitialized_data_size: u32,
    entry_point_rva: u32,
    base_of_code: u32,
    base_of_data: u32,
}

impl StandardFields {
    pub fn from(slice: &[u8; 28]) -> StandardFields {
        StandardFields {
            magic: u16::from_le_bytes(slice[0..2].try_into().unwrap()),
            l_major: slice[2],
            l_minor: slice[3],
            code_size: u32::from_le_bytes(slice[4..8].try_into().unwrap()),
            initialized_data_size: u32::from_le_bytes(slice[8..12].try_into().unwrap()),
            uninitialized_data_size: u32::from_le_bytes(slice[12..16].try_into().unwrap()),
            entry_point_rva: u32::from_le_bytes(slice[16..20].try_into().unwrap()),
            base_of_code: u32::from_le_bytes(slice[20..24].try_into().unwrap()),
            base_of_data: u32::from_le_bytes(slice[24..28].try_into().unwrap()),
        }
    }
}