/// # II.25.2.3.3 PE header data directories 
///
/// The optional header data directories give the address and size of several tables that appear in the 
/// sections of the PE file. Each data directory entry contains the RVA and Size of the structure it 
/// describes, in that order. 
/// 
/// | Offset | Size | Field                   | Description |
/// | ------ | ---- | ----------------------- | ----------- |
/// | 96     | 8    | Export Table            | Always 0 (§II.24.1). |
/// | 104    | 8    | Import Table            | RVA and Size of Import Table, (§II.25.3.1). |
/// | 112    | 8    | Resource Table          | Always 0 (§II.24.1). |
/// | 120    | 8    | Exception Table         | Always 0 (§II.24.1). |
/// | 128    | 8    | Certificate Table       | Always 0 (§II.24.1). |
/// | 136    | 8    | Base Relocation Table   | Relocation Table; set to 0 if unused (§). |
/// | 144    | 8    | Debug                   | Always 0 (§II.24.1). |
/// | 152    | 8    | Copyright               | Always 0 (§II.24.1). |
/// | 160    | 8    | Global Ptr              | Always 0 (§II.24.1). |
/// | 168    | 8    | TLS Table               | Always 0 (§II.24.1). |
/// | 176    | 8    | Load Config Table       | Always 0 (§II.24.1). |
/// | 184    | 8    | Bound Import            | Always 0 (§II.24.1). |
/// | 192    | 8    | IAT                     | RVA and Size of Import Address Table, (§II.25.3.1). |
/// | 200    | 8    | Delay Import Descriptor | Always 0 (§II.24.1). |
/// | 208    | 8    | CLI Header              | CLI Header with directories for runtime data, (§II.25.3.1). |
/// | 216    | 8    | Reserved                | Always 0 (§II.24.1). |
/// 
/// The tables pointed to by the directory entries are stored in one of the PE file’s sections; these sections 
/// themselves are described by section headers.  
pub struct DataDirectories {
    pub export_table: DataDirectory,
    pub import_table: DataDirectory,
    pub resource_table: DataDirectory,
    pub exception_table: DataDirectory,
    pub certificate_table: DataDirectory,
    pub base_relocation_table: DataDirectory,
    pub debug: DataDirectory,
    pub copyright: DataDirectory,
    pub global_ptr: DataDirectory,
    pub tls_table: DataDirectory,
    pub load_config_table: DataDirectory,
    pub bound_import: DataDirectory,
    pub iat: DataDirectory,
    pub delay_import_descriptor: DataDirectory,
    pub cli_header: DataDirectory,
    pub reserved: DataDirectory,
}

impl DataDirectories {
    fn read_directory(slice: &[u8], start: usize) -> DataDirectory {
        DataDirectory::from(&slice[start..start+8].try_into().unwrap())
    }

    pub fn from(slice: &[u8; 128]) -> DataDirectories {
        DataDirectories {
            export_table: Self::read_directory(slice, 0),
            import_table: Self::read_directory(slice, 8),
            resource_table: Self::read_directory(slice, 16),
            exception_table: Self::read_directory(slice, 24),
            certificate_table: Self::read_directory(slice, 32),
            base_relocation_table: Self::read_directory(slice, 40),
            debug: Self::read_directory(slice, 48),
            copyright: Self::read_directory(slice, 56),
            global_ptr: Self::read_directory(slice, 64),
            tls_table: Self::read_directory(slice, 72),
            load_config_table: Self::read_directory(slice, 80),
            bound_import: Self::read_directory(slice, 88),
            iat: Self::read_directory(slice, 96),
            delay_import_descriptor: Self::read_directory(slice, 104),
            cli_header: Self::read_directory(slice, 112),
            reserved: Self::read_directory(slice, 120),
        }
    }
}

pub struct DataDirectory {
    pub rva: u32,
    pub size: u32,
}

impl DataDirectory {
    pub fn new(rva: u32, size: u32) -> DataDirectory {
        DataDirectory {
            rva,
            size,
        }
    }

    pub fn from(slice: &[u8; 8]) -> DataDirectory {
        DataDirectory {
            rva: u32::from_le_bytes(slice[0..4].try_into().unwrap()),
            size: u32::from_le_bytes(slice[4..8].try_into().unwrap()),
        }
    }

    pub fn from_slice(slice: &[u8]) -> DataDirectory {
        DataDirectory {
            rva: u32::from_le_bytes(slice[0..4].try_into().unwrap()),
            size: u32::from_le_bytes(slice[4..8].try_into().unwrap()),
        }
    }
}