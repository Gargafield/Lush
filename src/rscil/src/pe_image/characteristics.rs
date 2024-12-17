/// # II.25.2.2.1 Characteristics
/// 
/// | Flag                         | Value   | Description                                                                 |
/// |------------------------------|---------|-----------------------------------------------------------------------------|
/// | IMAGE_FILE_RELOCS_STRIPPED   | 0x0001  | Shall be zero                                                               |
/// | IMAGE_FILE_EXECUTABLE_IMAGE  | 0x0002  | Shall be one                                                                |
/// | IMAGE_FILE_32BIT_MACHINE     | 0x0100  | Shall be one if and only if COMIMAGE_FLAGS_32BITREQUIRED is one (25.3.3.1)  |
/// | IMAGE_FILE_DLL               | 0x2000  | The image file is a dynamic-link library (DLL).                             |
/// 
/// For the flags not mentioned above, flags 0x0010, 0x0020, 0x0400 and 0x0800 are implementation specific, and all others should be zero (§II.24.1).
pub struct Characteristics(u16);

impl Characteristics {
    pub const IMAGE_FILE_RELOCS_STRIPPED : u16 = 0x0001;
    pub const IMAGE_FILE_EXECUTABLE_IMAGE : u16 = 0x0002;
    pub const IMAGE_FILE_32BIT_MACHINE : u16 = 0x0100;
    pub const IMAGE_FILE_DLL : u16 = 0x2000;

    pub fn new(value: u16) -> Characteristics {
        Characteristics(value)
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