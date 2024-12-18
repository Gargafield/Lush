/// # II.25.2.2.1 Characteristics
/// 
/// | Flag                           | Value    | Description |
/// | ------------------------------ | -------- | ----------- |
/// | `IMAGE_FILE_RELOCS_STRIPPED`   | `0x0001` | Shall be zero |
/// | `IMAGE_FILE_EXECUTABLE_IMAGE`  | `0x0002` | Shall be one |
/// | `IMAGE_FILE_32BIT_MACHINE`     | `0x0100` | Shall be one if and only if `COMIMAGE_FLAGS_32BITREQUIRED` is one (25.3.3.1) |
/// | `IMAGE_FILE_DLL`               | `0x2000` | The image file is a dynamic-link library (DLL). |
/// 
/// For the flags not mentioned above, flags 0x0010, 0x0020, 0x0400 and 0x0800 are implementation specific, and all others should be zero (§II.24.1).
pub struct FileCharacteristics(u16);

impl FileCharacteristics {
    pub const IMAGE_FILE_RELOCS_STRIPPED : u16 = 0x0001;
    pub const IMAGE_FILE_EXECUTABLE_IMAGE : u16 = 0x0002;
    pub const IMAGE_FILE_32BIT_MACHINE : u16 = 0x0100;
    pub const IMAGE_FILE_DLL : u16 = 0x2000;

    pub fn new(value: u16) -> FileCharacteristics {
        FileCharacteristics(value)
    }

    pub fn is_relocs_stripped(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_FILE_RELOCS_STRIPPED)
    }

    pub fn is_executable_image(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_FILE_EXECUTABLE_IMAGE)
    }

    pub fn is_32bit_machine(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_FILE_32BIT_MACHINE)
    }

    pub fn is_dll(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_FILE_DLL)
    }

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag != 0
    }
}

/// # II.25.3 Section headers 
/// 
/// [...]
/// 
/// The following table defines the possible characteristics of the section. 
/// 
/// | Flag                               | Value        | Description |
/// | ---------------------------------- | ------------ | ----------- |
/// | `IMAGE_SCN_CNT_CODE`               | `0x00000020` | Section contains code. |
/// | `IMAGE_SCN_CNT_INITIALIZED_DATA`   | `0x00000040` | Section contains initialized data. |
/// | `IMAGE_SCN_CNT_UNINITIALIZED_DATA` | `0x00000080` | Section contains uninitialized data. |
/// | `IMAGE_SCN_MEM_EXECUTE`            | `0x20000000` | Section can be executed as code. |
/// | `IMAGE_SCN_MEM_READ`               | `0x40000000` | Section can be read. |
/// | `IMAGE_SCN_MEM_WRITE`              | `0x80000000` | Section can be written to.|
pub struct SectionCharacteristics(u32);

impl SectionCharacteristics {
    pub const IMAGE_SCN_CNT_CODE : u32 = 0x00000020;
    pub const IMAGE_SCN_CNT_INITIALIZED_DATA : u32 = 0x00000040;
    pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA : u32 = 0x00000080;
    pub const IMAGE_SCN_MEM_EXECUTE : u32 = 0x20000000;
    pub const IMAGE_SCN_MEM_READ : u32 = 0x40000000;
    pub const IMAGE_SCN_MEM_WRITE : u32 = 0x80000000;

    pub fn new(value: u32) -> SectionCharacteristics {
        SectionCharacteristics(value)
    }

    pub fn is_code(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_SCN_CNT_CODE)
    }

    pub fn is_initialized_data(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_SCN_CNT_INITIALIZED_DATA)
    }

    pub fn is_uninitialized_data(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_SCN_CNT_UNINITIALIZED_DATA)
    }

    pub fn is_execute(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_SCN_MEM_EXECUTE)
    }

    pub fn is_read(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_SCN_MEM_READ)
    }

    pub fn is_write(&self) -> bool {
        Self::check_flag(&self, Self::IMAGE_SCN_MEM_WRITE)
    }

    pub fn check_flag(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }
}