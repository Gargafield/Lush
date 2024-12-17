
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
    export_table: (u32, u32),
    import_table: (u32, u32),
    resource_table: (u32, u32),
    exception_table: (u32, u32),
    certificate_table: (u32, u32),
    base_relocation_table: (u32, u32),
    debug: (u32, u32),
    copyright: (u32, u32),
    global_ptr: (u32, u32),
    tls_table: (u32, u32),
    load_config_table: (u32, u32),
    bound_import: (u32, u32),
    iat: (u32, u32),
    delay_import_descriptor: (u32, u32),
    cli_header: (u32, u32),
    reserved: (u32, u32),
}

impl DataDirectories {
    fn read_pair(slice: &[u8; 8]) -> (u32, u32) {
        (u32::from_le_bytes(slice[0..4].try_into().unwrap()), u32::from_le_bytes(slice[4..8].try_into().unwrap()))
    }

    pub fn from(slice: &[u8; 128]) -> DataDirectories {
        DataDirectories {
            export_table: Self::read_pair(slice[0..8].try_into().unwrap()),
            import_table: Self::read_pair(slice[8..16].try_into().unwrap()),
            resource_table: Self::read_pair(slice[16..24].try_into().unwrap()),
            exception_table: Self::read_pair(slice[24..32].try_into().unwrap()),
            certificate_table: Self::read_pair(slice[32..40].try_into().unwrap()),
            base_relocation_table: Self::read_pair(slice[40..48].try_into().unwrap()),
            debug: Self::read_pair(slice[48..56].try_into().unwrap()),
            copyright: Self::read_pair(slice[56..64].try_into().unwrap()),
            global_ptr: Self::read_pair(slice[64..72].try_into().unwrap()),
            tls_table: Self::read_pair(slice[72..80].try_into().unwrap()),
            load_config_table: Self::read_pair(slice[80..88].try_into().unwrap()),
            bound_import: Self::read_pair(slice[88..96].try_into().unwrap()),
            iat: Self::read_pair(slice[96..104].try_into().unwrap()),
            delay_import_descriptor: Self::read_pair(slice[104..112].try_into().unwrap()),
            cli_header: Self::read_pair(slice[112..120].try_into().unwrap()),
            reserved: Self::read_pair(slice[120..128].try_into().unwrap()),
        }
    }
}