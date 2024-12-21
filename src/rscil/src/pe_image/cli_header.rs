use super::data_directories::DataDirectory;

/// # II.25.3.3 CLI header 
///
/// The CLI header contains all of the runtime-specific data entries and other information. The header 
/// should be placed in a read-only, sharable section of the image. This header is defined as follows:
/// 
/// | Offset | Size | Field                     | Description |
/// | ------ | ---- | ------------------------- | ----------- |
/// | 0      | 4    | Cb                        | Size of the header in bytes |
/// | 4      | 2    | MajorRuntimeVersion       | The minimum version of the runtime required to run this program, currently 2. |
/// | 6      | 2    | MinorRuntimeVersion       | The minor portion of the version, currently 0. |
/// | 8      | 8    | MetaData                  | RVA and size of the physical metadata (§II.24). |
/// | 16     | 4    | Flags                     | Flags describing this runtime image (§II.25.3.3.1). |
/// | 20     | 4    | EntryPointToken           | Token for the *MethodDef* or File of the entry point for the image |
/// | 24     | 8    | Resources                 | RVA and size of implementation-specific resources. |
/// | 32     | 8    | StrongNameSignature       | RVA of the hash data for this PE file used by the CLI loader for binding and versioning |
/// | 40     | 8    | CodeManagerTable          | Always 0 (§II.24.1). |
/// | 48     | 8    | VTableFixups              | RVA of an array of locations in the file that contain an array of function pointers (e.g., vtable slots), see below. |
/// | 56     | 8    | ExportAddressTableJumps   | Always 0 (§II.24.1). |
/// | 64     | 8    | ManagedNativeHeader       | Always 0 (§II.24.1). |
pub struct CliHeader {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub meta_data: DataDirectory,
    pub flags: RuntimeFlags,
    pub entry_point_token: u32,
    pub resources: DataDirectory,
    pub strong_name_signature: DataDirectory,
    pub code_manager_table: DataDirectory,
    pub vtable_fixups: DataDirectory,
    pub export_address_table_jumps: DataDirectory,
    pub managed_native_header: DataDirectory,
}

impl CliHeader {
    pub fn from(slice: &[u8; 72]) -> CliHeader {
        CliHeader {
            cb: u32::from_le_bytes(slice[0..4].try_into().unwrap()),
            major_runtime_version: u16::from_le_bytes(slice[4..6].try_into().unwrap()),
            minor_runtime_version: u16::from_le_bytes(slice[6..8].try_into().unwrap()),
            meta_data: DataDirectory::from_slice(&slice[8..16]),
            flags: RuntimeFlags::new(u32::from_le_bytes(slice[16..20].try_into().unwrap())),
            entry_point_token: u32::from_le_bytes(slice[20..24].try_into().unwrap()),
            resources: DataDirectory::from_slice(&slice[24..32]),
            strong_name_signature: DataDirectory::from_slice(&slice[32..40]),
            code_manager_table: DataDirectory::from_slice(&slice[40..48]),
            vtable_fixups: DataDirectory::from_slice(&slice[48..56]),
            export_address_table_jumps: DataDirectory::from_slice(&slice[56..64]),
            managed_native_header: DataDirectory::from_slice(&slice[64..72]),
        }
    }
}

/// # II.25.3.3.1 Runtime flags 
/// 
/// The following flags describe this runtime image and are used by the loader. All unspecified bits should 
/// be zero.
/// 
/// | Flag                               | Value        | Description |
/// | ---------------------------------- | ------------ | ----------- |
/// | `COMIMAGE_FLAGS_ILONLY`            | `0x00000001` | Shall be 1. |
/// | `COMIMAGE_FLAGS_32BITREQUIRED`     | `0x00000002` | Image can only be loaded into a 32-bit process, for instance if there are 32-bit vtablefixups, or casts from native integers to int32. CLI implementations that have 64-bit native integers shall refuse loading binaries with this flag set. |
/// | `COMIMAGE_FLAGS_STRONGNAMESIGNED`  | `0x00000008` | Image has a strong name signature. |
/// | `COMIMAGE_FLAGS_NATIVE_ENTRYPOINT` | `0x00000010` | Shall be 0. |
/// | `COMIMAGE_FLAGS_TRACKDEBUGDATA`    | `0x00010000` | Should be 0 (§II.24.1). |
pub struct RuntimeFlags(u32);

impl RuntimeFlags {
    pub const COMIMAGE_FLAGS_ILONLY : u32 = 0x00000001;
    pub const COMIMAGE_FLAGS_32BITREQUIRED : u32 = 0x00000002;
    pub const COMIMAGE_FLAGS_STRONGNAMESIGNED : u32 = 0x00000008;
    pub const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT : u32 = 0x00000010;
    pub const COMIMAGE_FLAGS_TRACKDEBUGDATA : u32 = 0x00010000;

    pub fn new(value: u32) -> RuntimeFlags {
        RuntimeFlags(value)
    }

    pub fn is_ilonly(&self) -> bool {
        self.check_flag(Self::COMIMAGE_FLAGS_ILONLY)
    }

    pub fn is_32bit_required(&self) -> bool {
        self.check_flag(Self::COMIMAGE_FLAGS_32BITREQUIRED)
    }

    pub fn is_strong_name_signed(&self) -> bool {
        self.check_flag(Self::COMIMAGE_FLAGS_STRONGNAMESIGNED)
    }

    pub fn is_native_entrypoint(&self) -> bool {
        self.check_flag(Self::COMIMAGE_FLAGS_NATIVE_ENTRYPOINT)
    }

    pub fn is_track_debug_data(&self) -> bool {
        self.check_flag(Self::COMIMAGE_FLAGS_TRACKDEBUGDATA)
    }
    
    pub fn check_flag(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }
}

