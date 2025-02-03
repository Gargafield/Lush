
use std::io::BufRead;

use super::*;

/// # [II.25.2.2] PE file header 
/// 
/// Immediately after the PE signature is the PE File header consisting of the following: 
/// 
/// | Offset | Size | Field                   | Description |
/// | ------ | ---- | ----------------------- | ----------- |
/// | 0x00   | 2    | Machine                 | Always `0x14c`. |
/// | 0x02   | 2    | Number of Sections      | Number of sections; indicates size of the Section Table, which immediately follows the headers. |
/// | 0x04   | 4    | Time/Date Stamp         | Time and date the file was created in seconds since `January 1st 1970 00:00:00` or `0`. |
/// | 0x08   | 4    | Pointer to Symbol Table | Always `0` ([§II.24.1]). |
/// | 0x0c   | 4    | Number of Symbols       | Always `0` ([§II.24.1]). |
/// | 0x10   | 2    | Optional Header Size    | Size of the optional header, the format is described below. |
/// | 0x12   | 2    | Characteristics         | Flags indicating attributes of the file, see [`FileCharacteristics`]. |
/// 
/// [II.25.2.2]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=304
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
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

/// # [II.24.2] File headers
/// ## [II.24.2.1] Metadata root
/// 
/// The root of the physical metadata starts with a magic signature, several bytes of version and other 
/// miscellaneous information, followed by a count and an array of stream headers, one for each stream 
/// that is present. The actual encoded tables and heaps are stored in the streams, which immediately 
/// follow this array of headers. 
/// 
/// | Offset       | Size     | Field         | Description |
/// | ------------ | -------- | -----------   | ----------- |
/// | 0            | 4        | Signature     | Magic signature for physical metadata : `0x424A5342`. |
/// | 4            | 2        | MajorVersion  | Major version, 1 (ignore on read) |
/// | 6            | 2        | MinorVersion  | Minor version, 1 (ignore on read) |
/// | 8            | 4        | Reserved      | Reserved, always 0 ([§II.24.1]). |
/// | 12           | 4        | Length        | Number of bytes allocated to hold version string (including null terminator), call this *x*. Call the length of the string (including the terminator) *m* (we require *m* <= 255); the length *x* is *m* rounded up to a multiple of four. |
/// | 16           | *m*      | Version       | UTF8-encoded null-terminated version string of length *m* (see above) |
/// | 16+*m*       | *x*-*m*  | Padding       | Padding to next 4 byte boundary. |
/// | 16+*x*       | 2        | Flags         | Reserved, always 0 ([§II.24.1]). |
/// | 16+*x*+2     | 2        | Streams       | Number of streams, say *n*. |
/// | 16+*x*+4     | -        | StreamHeaders | Array of *n* [`StreamHeader`] structures. |
/// 
/// [II.24.2]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
/// [II.24.2.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
pub struct MetadataHeader {
    pub signature: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub reserved: u32,
    pub length: u32,
    pub version: String,
    pub flags: u16,
    pub streams: u16,
    pub stream_headers: Vec<StreamHeader>,
}

impl MetadataHeader {
    pub fn from(buffer: &mut Buffer) -> Result<MetadataHeader, std::io::Error> {
        let signature = buffer.read_u32::<LittleEndian>()?;

        // See Description of Signature field in the table above
        assert!(signature == 0x424A5342, "Invalid metadata signature: 0x{:X}", signature);

        let major_version = buffer.read_u16::<LittleEndian>()?;
        let minor_version = buffer.read_u16::<LittleEndian>()?;
        let reserved = buffer.read_u32::<LittleEndian>()?;
        let length = buffer.read_u32::<LittleEndian>()?;

        let mut version = vec![0u8; length as usize];
        buffer.read_exact(&mut version)?;
        let version = String::from_utf8(version).unwrap();

        let mut padding = vec![0u8; (length % 4) as usize];
        buffer.read_exact(&mut padding)?;

        let flags = buffer.read_u16::<LittleEndian>()?;
        let streams = buffer.read_u16::<LittleEndian>()?;

        let mut stream_headers = Vec::with_capacity(streams as usize);
        for _ in 0..streams {
            stream_headers.push(StreamHeader::from(buffer)?);
        }

        Ok(MetadataHeader {
            signature,
            major_version,
            minor_version,
            reserved,
            length,
            version,
            flags,
            streams,
            stream_headers,
        })
    }
}

/// # [II.24.2.2] Stream header
/// 
/// A stream header gives the names, and the position and length of a particular table or heap. Note that the 
/// length of a Stream header structure is not fixed, but depends on the length of its name field (a variable 
/// length null-terminated string).
/// 
/// | Offset | Size | Field  | Description |
/// | ------ | ---- | ------ | ----------- |
/// | 0      | 4    | Offset | Memory offset to start of this stream from start of the metadata root ([`MetadataHeader`]) | 
/// | 4      | 4    | Size   | Size of this stream in bytes, shall be a multiple of 4. |
/// | 8      | -    | Name   | Name of the stream as null-terminated variable length array of ASCII characters, padded to the next 4-byte boundary with `\0` characters. The name is limited to 32 characters. |
/// 
/// [II.24.2.2]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=298
pub struct StreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}

impl StreamHeader {
    pub fn from(buffer: &mut Buffer) -> Result<StreamHeader, std::io::Error> {
        let offset = buffer.read_u32::<LittleEndian>()?;
        let size = buffer.read_u32::<LittleEndian>()?;

        let mut name = Vec::new();
        buffer.read_until(0, &mut name)?;
        
        // Padding to the next 4-byte boundary
        let padding = 4 - (name.len() % 4);
        let mut padding = vec![0u8; padding];
        buffer.read_exact(&mut padding)?;
        
        name.pop(); // Remove the null terminator
        let name = String::from_utf8(name).unwrap();

        Ok(StreamHeader {
            offset,
            size,
            name,
        })
    }
}

/// # [II.25.2.3] PE optional header 
/// 
/// Immediately after the PE Header is the PE Optional Header. This header contains the following information: 
/// 
/// | Offset | Size | Header part         | Description |
/// |--------|------|---------------------|-------------|
/// | 0      | 28   | Standard fields     | These define general properties of the PE file, see [`StandardFields`]. |
/// | 28     | 68   | NT-specific fields  | These include additional fields to support specific features of Windows, see [`NtSpecificFields`]. |
/// | 96     | 128  | Data directories    | These fields are address/size pairs for special tables, found in the image file (for example, Import Table and Export Table). |
/// 
/// [II.25.2.3]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=305
pub struct PeOptionalHeader {
    pub standard_fields: StandardFields,
    pub nt_specific_fields: NtSpecificFields,
    pub data_directories: DataDirectories,
}

impl PeOptionalHeader {
    pub const SIZE: usize = 224;

    pub fn from(slice: &[u8; 224]) -> PeOptionalHeader {
        PeOptionalHeader {
            standard_fields: StandardFields::from(&slice[0..28].try_into().unwrap()),
            nt_specific_fields: NtSpecificFields::from(&slice[28..96].try_into().unwrap()),
            data_directories: DataDirectories::from(&slice[96..224].try_into().unwrap()),
        }
    }
}

/// # [II.25.2.3.1] PE header standard fields 
/// 
/// These fields are required for all PE files and contain the following information: 
/// 
/// | Offset | Size | Field         | Description |
/// | ------ | ---- | ------------- | ----------- |
/// | 0      | 2    | Magic         | Always `0x10B`. |
/// | 2      | 1    | LMajor        | Always `6` ([§II.24.1]). |
/// | 3      | 1    | LMinor        | Always `0` ([§II.24.1]). |
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
/// 
/// [II.25.2.3.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=305
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
pub struct StandardFields {
    pub magic: u16,
    pub l_major: u8,
    pub l_minor: u8,
    pub code_size: u32,
    pub initialized_data_size: u32,
    pub uninitialized_data_size: u32,
    pub entry_point_rva: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
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

/// # [II.25.2.3.2] PE header Windows NT-specific fields 
///
/// These fields are Windows NT specific: 
///
/// | Offset | Size | Field                      | Description |
/// | ------ | ---- | -------------------------- | ----------- |
/// | 28     | 4    | Image                      | Base Shall be a multiple of `0x10000`. |
/// | 32     | 4    | Section                    | Alignment Shall be greater than File Alignment. |
/// | 36     | 4    | File Alignment             | Should be `0x200` ([§II.24.1]). |
/// | 40     | 2    | OS Major                   | Should be 5 ([§II.24.1]). |
/// | 42     | 2    | OS Minor                   | Should be 0 ([§II.24.1]). |
/// | 44     | 2    | User Major                 | Should be 0 ([§II.24.1]). |
/// | 46     | 2    | User Minor                 | Should be 0 ([§II.24.1]). |
/// | 48     | 2    | SubSys Major               | Should be 5 ([§II.24.1]). |
/// | 50     | 2    | SubSys Minor               | Should be 0 ([§II.24.1]). |
/// | 52     | 4    | Reserved                   | Shall be zero |
/// | 56     | 4    | Image Size                 | Size, in bytes, of image, including all headers and padding; shall be a multiple of Section Alignment. |
/// | 60     | 4    | Header Size                | Combined size of MS-DOS Header, PE Header, PE Optional Header and padding; shall be a multiple of the file alignment. |
/// | 64     | 4    | File Checksum              | Should be 0 ([§II.24.1]). |
/// | 68     | 2    | SubSystem                  | Subsystem required to run this image. Shall be either `IMAGE_SUBSYSTEM_WINDOWS_CUI` (`0x3`) or `IMAGE_SUBSYSTEM_WINDOWS_GUI` (`0x2`). |
/// | 70     | 2    | DLL Flags                  | Bits `0x100f` shall be zero. |
/// | 72     | 4    | Stack Reserve Size         | Should be `0x100000` (1Mb) ([§II.24.1]). |
/// | 76     | 4    | Stack Commit Size          | Should be `0x1000` (4Kb) ([§II.24.1]). |
/// | 80     | 4    | Heap Reserve Size          | Should be `0x100000` (1Mb) ([§II.24.1]). |
/// | 84     | 4    | Heap Commit Size           | Should be `0x1000` (4Kb) ([§II.24.1]). |
/// | 88     | 4    | Loader Flags               | Shall be 0 |
/// | 92     | 4    | Number of Data Directories | Shall be `0x10` |
/// 
/// [II.25.2.3.2]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=305
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
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

/// # [II.25.2.3.3] PE header data directories 
///
/// The optional header data directories give the address and size of several tables that appear in the 
/// sections of the PE file. Each data directory entry contains the RVA and Size of the structure it 
/// describes, in that order. 
/// 
/// | Offset | Size | Field                   | Description |
/// | ------ | ---- | ----------------------- | ----------- |
/// | 96     | 8    | Export Table            | Always 0 ([§II.24.1]). |
/// | 104    | 8    | Import Table            | RVA and Size of Import Table, ([§II.25.3.1]). |
/// | 112    | 8    | Resource Table          | Always 0 ([§II.24.1]). |
/// | 120    | 8    | Exception Table         | Always 0 ([§II.24.1]). |
/// | 128    | 8    | Certificate Table       | Always 0 ([§II.24.1]). |
/// | 136    | 8    | Base Relocation Table   | Relocation Table; set to 0 if unused (§). |
/// | 144    | 8    | Debug                   | Always 0 ([§II.24.1]). |
/// | 152    | 8    | Copyright               | Always 0 ([§II.24.1]). |
/// | 160    | 8    | Global Ptr              | Always 0 ([§II.24.1]). |
/// | 168    | 8    | TLS Table               | Always 0 ([§II.24.1]). |
/// | 176    | 8    | Load Config Table       | Always 0 ([§II.24.1]). |
/// | 184    | 8    | Bound Import            | Always 0 ([§II.24.1]). |
/// | 192    | 8    | IAT                     | RVA and Size of Import Address Table, ([§II.25.3.1]). |
/// | 200    | 8    | Delay Import Descriptor | Always 0 ([§II.24.1]). |
/// | 208    | 8    | CLI Header              | CLI Header with directories for runtime data, ([§II.25.3.1]). |
/// | 216    | 8    | Reserved                | Always 0 ([§II.24.1]). |
/// 
/// The tables pointed to by the directory entries are stored in one of the PE file’s sections; these sections 
/// themselves are described by section headers.  
/// 
/// [§II.25.3.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=305
/// [II.25.2.3.3]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=306
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
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

/// # [II.25.3] Section headers 
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
/// | 24     | 4    | PointerToRelocations  | Should be 0 ([§II.24.1]). |
/// | 28     | 4    | PointerToLinenumbers  | Should be 0 ([§II.24.1]). |
/// | 32     | 2    | NumberOfRelocations   | Should be 0 ([§II.24.1]). |
/// | 34     | 2    | NumberOfLinenumbers   | Should be 0 ([§II.24.1]). |
/// | 36     | 4    | Characteristics       | Flags describing section's characteristics; see below. |
/// 
/// [II.25.3]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=307
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
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

/// # [II.25.3.3] CLI header 
///
/// The CLI header contains all of the runtime-specific data entries and other information. The header 
/// should be placed in a read-only, sharable section of the image. This header is defined as follows:
/// 
/// | Offset | Size | Field                     | Description |
/// | ------ | ---- | ------------------------- | ----------- |
/// | 0      | 4    | Cb                        | Size of the header in bytes |
/// | 4      | 2    | MajorRuntimeVersion       | The minimum version of the runtime required to run this program, currently 2. |
/// | 6      | 2    | MinorRuntimeVersion       | The minor portion of the version, currently 0. |
/// | 8      | 8    | MetaData                  | RVA and size of the physical metadata ([§II.24]). |
/// | 16     | 4    | Flags                     | Flags describing this runtime image ([`RuntimeFlags`]). |
/// | 20     | 4    | EntryPointToken           | Token for the [`MethodDef`] or [`File`](crate::file) of the entry point for the image |
/// | 24     | 8    | Resources                 | RVA and size of implementation-specific resources. |
/// | 32     | 8    | StrongNameSignature       | RVA of the hash data for this PE file used by the CLI loader for binding and versioning |
/// | 40     | 8    | CodeManagerTable          | Always 0 ([§II.24.1]). |
/// | 48     | 8    | VTableFixups              | RVA of an array of locations in the file that contain an array of function pointers (e.g., vtable slots), see below. |
/// | 56     | 8    | ExportAddressTableJumps   | Always 0 ([§II.24.1]). |
/// | 64     | 8    | ManagedNativeHeader       | Always 0 ([§II.24.1]). |
/// 
/// [II.25.3.3]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=309
/// [§II.24]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
/// [§II.24.1]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=297
pub struct CliHeader {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub meta_data: DataDirectory,
    pub flags: RuntimeFlags,
    pub entry_point_token: MetadataToken,
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
            entry_point_token: MetadataToken::from_raw(u32::from_le_bytes(slice[20..24].try_into().unwrap())),
            resources: DataDirectory::from_slice(&slice[24..32]),
            strong_name_signature: DataDirectory::from_slice(&slice[32..40]),
            code_manager_table: DataDirectory::from_slice(&slice[40..48]),
            vtable_fixups: DataDirectory::from_slice(&slice[48..56]),
            export_address_table_jumps: DataDirectory::from_slice(&slice[56..64]),
            managed_native_header: DataDirectory::from_slice(&slice[64..72]),
        }
    }
}
