use super::{data_directories::DataDirectories, nt_specific_fields::NtSpecificFields, standard_fields::StandardFields};


/// # II.25.2.3 PE optional header 
/// 
/// Immediately after the PE Header is the PE Optional Header. This header contains the following information: 
/// 
/// | Offset | Size | Header part         | Description |
/// |--------|------|---------------------|-------------|
/// | 0      | 28   | Standard fields     | These define general properties of the PE file, see §II.25.2.3.1. |
/// | 28     | 68   | NT-specific fields  | These include additional fields to support specific features of Windows, see II.25.2.3.2. |
/// | 96     | 128  | Data directories    | These fields are address/size pairs for special tables, found in the image file (for example, Import Table and Export Table). |
pub struct PeOptionalHeader {
    standard_fields: StandardFields,
    nt_specific_fields: NtSpecificFields,
    data_directories: DataDirectories,
}

impl PeOptionalHeader {
    pub fn from(slice: &[u8; 224]) -> PeOptionalHeader {
        PeOptionalHeader {
            standard_fields: StandardFields::from(&slice[0..28].try_into().unwrap()),
            nt_specific_fields: NtSpecificFields::from(&slice[28..96].try_into().unwrap()),
            data_directories: DataDirectories::from(&slice[96..224].try_into().unwrap()),
        }
    }
}