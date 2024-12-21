
/// # II.25.2.3.2 PE header Windows NT-specific fields 
///
/// These fields are Windows NT specific: 
///
/// | Offset | Size | Field                      | Description |
/// | ------ | ---- | -------------------------- | ----------- |
/// | 28     | 4    | Image                      | Base Shall be a multiple of 0x10000. |
/// | 32     | 4    | Section                    | Alignment Shall be greater than File Alignment. |
/// | 36     | 4    | File Alignment             | Should be 0x200 (§II.24.1). |
/// | 40     | 2    | OS Major                   | Should be 5 (§II.24.1). |
/// | 42     | 2    | OS Minor                   | Should be 0 (§II.24.1). |
/// | 44     | 2    | User Major                 | Should be 0 (§II.24.1). |
/// | 46     | 2    | User Minor                 | Should be 0 (§II.24.1). |
/// | 48     | 2    | SubSys Major               | Should be 5 (§II.24.1). |
/// | 50     | 2    | SubSys Minor               | Should be 0 (§II.24.1). |
/// | 52     | 4    | Reserved                   | Shall be zero |
/// | 56     | 4    | Image Size                 | Size, in bytes, of image, including all headers and padding; shall be a multiple of Section Alignment. |
/// | 60     | 4    | Header Size                | Combined size of MS-DOS Header, PE Header, PE Optional Header and padding; shall be a multiple of the file alignment. |
/// | 64     | 4    | File Checksum              | Should be 0 (§II.24.1). |
/// | 68     | 2    | SubSystem                  | Subsystem required to run this image. Shall be either IMAGE_SUBSYSTEM_WINDOWS_CUI (0x3) or IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2). |
/// | 70     | 2    | DLL Flags                  | Bits 0x100f shall be zero. |
/// | 72     | 4    | Stack Reserve Size         | Should be 0x100000 (1Mb) (§II.24.1). |
/// | 76     | 4    | Stack Commit Size          | Should be 0x1000 (4Kb) (§II.24.1). |
/// | 80     | 4    | Heap Reserve Size          | Should be 0x100000 (1Mb) (§II.24.1). |
/// | 84     | 4    | Heap Commit Size           | Should be 0x1000 (4Kb) (§II.24.1). |
/// | 88     | 4    | Loader Flags               | Shall be 0 |
/// | 92     | 4    | Number of Data Directories | Shall be 0x10 |
pub struct NtSpecificFields {
    pub image: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub os_major: u16,
    pub os_minor: u16,
    pub user_major: u16,
    pub user_minor: u16,
    pub sub_sys_major: u16,
    pub sub_sys_minor: u16,
    pub reserved: u32,
    pub image_size: u32,
    pub header_size: u32,
    pub file_checksum: u32,
    pub sub_system: u16,
    pub dll_flags: u16,
    pub stack_reserve_size: u32,
    pub stack_commit_size: u32,
    pub heap_reserve_size: u32,
    pub heap_commit_size: u32,
    pub loader_flags: u32,
    pub number_of_data_directories: u32,
}

impl NtSpecificFields {
    pub fn from(slice: &[u8; 68]) -> NtSpecificFields {
        NtSpecificFields {
            image: u32::from_le_bytes(slice[0..4].try_into().unwrap()),
            section_alignment: u32::from_le_bytes(slice[4..8].try_into().unwrap()),
            file_alignment: u32::from_le_bytes(slice[8..12].try_into().unwrap()),
            os_major: u16::from_le_bytes(slice[12..14].try_into().unwrap()),
            os_minor: u16::from_le_bytes(slice[14..16].try_into().unwrap()),
            user_major: u16::from_le_bytes(slice[16..18].try_into().unwrap()),
            user_minor: u16::from_le_bytes(slice[18..20].try_into().unwrap()),
            sub_sys_major: u16::from_le_bytes(slice[20..22].try_into().unwrap()),
            sub_sys_minor: u16::from_le_bytes(slice[22..24].try_into().unwrap()),
            reserved: u32::from_le_bytes(slice[24..28].try_into().unwrap()),
            image_size: u32::from_le_bytes(slice[28..32].try_into().unwrap()),
            header_size: u32::from_le_bytes(slice[32..36].try_into().unwrap()),
            file_checksum: u32::from_le_bytes(slice[36..40].try_into().unwrap()),
            sub_system: u16::from_le_bytes(slice[40..42].try_into().unwrap()),
            dll_flags: u16::from_le_bytes(slice[42..44].try_into().unwrap()),
            stack_reserve_size: u32::from_le_bytes(slice[44..48].try_into().unwrap()),
            stack_commit_size: u32::from_le_bytes(slice[48..52].try_into().unwrap()),
            heap_reserve_size: u32::from_le_bytes(slice[52..56].try_into().unwrap()),
            heap_commit_size: u32::from_le_bytes(slice[56..60].try_into().unwrap()),
            loader_flags: u32::from_le_bytes(slice[60..64].try_into().unwrap()),
            number_of_data_directories: u32::from_le_bytes(slice[64..68].try_into().unwrap()),
        }
    }
}
