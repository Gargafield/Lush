use std::{collections::HashMap, io::BufRead};

use streams::HeapSizes;

use super::*;

pub struct PeParser {
    filename : String,
    buffer : BufReader<File>,
    row_count: HashMap<TableKind, u32>,
    heap_sizes: HeapSizes,
}

// II.25.2.1 MS-DOS header
static DOS_STUB_SIZE: usize = 128;
static DOS_STUB : [u8; 128] = [
    // Part 1
    0x4d, 0x5a, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 
    0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 
    0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00,
    // lfanew
    0x00, 0x00, 0x00, 0x00, 
    // Part 2
    0x0e, 0x1f, 0xba, 0x0e, 0x00, 0xb4, 0x09, 0xcd, 
    0x21, 0xb8, 0x01, 0x4c, 0xcd, 0x21, 0x54, 0x68, 
    0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 
    0x61, 0x6d, 0x20, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 
    0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6e, 
    0x20, 0x69, 0x6e, 0x20, 0x44, 0x4f, 0x53, 0x20, 
    0x6d, 0x6f, 0x64, 0x65, 0x2e, 0x0d, 0x0d, 0x0a, 
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];
// "PE\0\0"
static PE_SIGNATURE : [u8; 4] = [0x50, 0x45, 0x00, 0x00];

impl PeParser {
    pub fn new(filename: &str, buffer: BufReader<File>) -> PeParser {
        PeParser {
            filename: filename.to_string(),
            buffer,
            row_count: HashMap::new(),
            heap_sizes: HeapSizes::from(0u8),
        }
    }

    pub fn open(filename: &str) -> Result<PeParser, std::io::Error> {
        let file = File::open(filename)?;
        let buffer = BufReader::new(file);
        Ok(PeParser::new(filename, buffer))
    }

    pub fn read(&mut self) -> Result<PeImage, std::io::Error> {
        self.buffer.seek(SeekFrom::Start(0))?;

        self.read_dos_stub()?;
        let header = self.read_pe_header()?;

        // See Description of Machine field at II.25.2.2 PE file header
        assert!(header.machine == 0x14c, "Invalid machine type");

        let optional_header = self.read_pe_optional_header(&header)?;
        let sections = self.read_section_header(&header)?;
        let cli_header = self.read_cli_header(&optional_header, &sections)?;
        let metadata_header = self.read_metadata_header(&cli_header, &sections)?;
        let streams = self.read_streams(
            Self::get_address(&sections, cli_header.meta_data.rva),
            &metadata_header.stream_headers
        )?;

        Ok(PeImage {
            filename: self.filename.clone(),
            pe_header: header,
            optional_header,
            sections,
            cli_header,
            metadata_header,
            streams,
        })
    }

    /// # II.25.2.1 MS-DOS header
    /// The PE format starts with an MS-DOS stub of exactly the following **128** bytes to be placed at the front 
    /// of the module. At offset `0x3c` in the DOS header is a 4-byte unsigned integer offset, `lfanew`, to the PE 
    /// signature (shall be "PE\0\0"), immediately followed by the PE file header.
    fn read_dos_stub(&mut self) -> Result<(), std::io::Error> {
        let mut header = [0u8; DOS_STUB_SIZE];
        self.buffer.read_exact(&mut header)?;

        if header[..0x3c] != DOS_STUB[..0x3c] || header[0x40..] != DOS_STUB[0x40..] {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid DOS header"));
        }

        // Check if the PE signature is present
        let mut signature = [0u8; 4];
        self.buffer.read_exact(&mut signature)?;
        if signature != PE_SIGNATURE {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid PE signature"));
        }

        Ok(())
    }

    /// # II.25.2.2 PE file header 
    /// See [`PeHeader`] struct for more information.
    fn read_pe_header(&mut self) -> Result<PeHeader, std::io::Error> {
        let mut header = [0u8; 20];
        self.buffer.read_exact(&mut header)?;
        Ok(PeHeader::from(&header))
    }

    /// II.25.2.3 PE optional header 
    /// See [`PeOptionalHeader`] struct for more information.
    fn read_pe_optional_header(&mut self, header: &PeHeader) -> Result<PeOptionalHeader, std::io::Error> {
        let mut buffer = vec![0u8; header.optional_header_size as usize];
        self.buffer.read_exact(&mut buffer)?;
        Ok(PeOptionalHeader::from(&buffer[..224].try_into().unwrap()))
    }

    /// # II.25.3 Section headers 
    /// See [`SectionHeader`] struct for more information.
    fn read_section_header(&mut self, header: &PeHeader) -> Result<Vec<section_header::SectionHeader>, std::io::Error> {
        let mut sections = Vec::with_capacity(header.number_of_sections as usize);
        for _ in 0..header.number_of_sections {
            let mut buffer = [0u8; 40];
            self.buffer.read_exact(&mut buffer)?;
            sections.push(section_header::SectionHeader::from(&buffer));
        }
        Ok(sections)
    }

    /// # II.25.3.3 CLI header 
    /// See [`CliHeader`] struct for more information.
    fn read_cli_header(&mut self, optional_header: &PeOptionalHeader, sections: &Vec<SectionHeader>) -> Result<CliHeader, std::io::Error> {
        self.seek_rva(sections, optional_header.data_directories.cli_header.rva);
        let mut buffer = [0u8; 72];
        self.buffer.read_exact(&mut buffer)?;
        Ok(CliHeader::from(&buffer))
    }

    /// ## II.24.2.1 Metadata root
    /// See [`MetadataHeader`] struct for more information.
    fn read_metadata_header(&mut self, cli_header: &CliHeader, sections: &Vec<SectionHeader>) -> Result<MetadataHeader, std::io::Error> {
        self.seek_rva(sections, cli_header.meta_data.rva);
        MetadataHeader::from(self)
    }

    /// # II.24.2.2 Stream header
    /// See [`Streams`] struct for more information.
    fn read_streams(&mut self, root_address: u64, headers: &Vec<StreamHeader>) -> Result<streams::Streams, std::io::Error> {
        Streams::from(self, root_address, headers)
    }

    /// II.25 File format extensions to PE 
    /// 
    /// [...]
    /// 
    /// The PE format frequently uses the term RVA (Relative Virtual Address). An RVA is the address of an 
    /// item once loaded into memory, with the base address of the image file subtracted from it (i.e., the offset 
    /// from the base address where the file is loaded). The RVA of an item will almost always differ from its 
    /// position within the file on disk. To compute the file position of an item with RVA r, search all the 
    /// sections in the PE file to find the section with RVA s, length l and file position p in which the RVA 
    /// lies, ie s  r < s+l. The file position of the item is then given by p+(r-s). 
    fn seek_rva(&mut self, sections: &Vec<SectionHeader>, rva: u32) {
        self.buffer.seek(SeekFrom::Start(Self::get_address(sections, rva))).unwrap();
    }

    fn get_address(sections: &Vec<SectionHeader>, rva: u32) -> u64 {
        for section in sections {
            if rva >= section.virtual_address && rva < section.virtual_address + section.virtual_size {
                return section.pointer_to_raw_data as u64 + (rva - section.virtual_address) as u64;
            }
        }
        panic!("RVA not found in any section");
    }

    pub fn seek(&mut self, position: u64) -> std::io::Result<u64> {
        self.buffer.seek(SeekFrom::Start(position))
    }

    pub fn read_u64(&mut self) -> Result<u64, std::io::Error> {
        let mut buffer = [0u8; 8];
        self.buffer.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    pub fn read_u32(&mut self) -> Result<u32, std::io::Error> {
        let mut buffer = [0u8; 4];
        self.buffer.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_u16(&mut self) -> Result<u16, std::io::Error> {
        let mut buffer = [0u8; 2];
        self.buffer.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    pub fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut buffer = [0u8; 1];
        self.buffer.read_exact(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If e is an index into the GUID heap, 'blob', or String heap, it is stored using the number of bytes as defined in the HeapSizes field.
    pub fn read_string_index(&mut self) -> Result<StringIndex, std::io::Error> {
        if self.heap_sizes.string_size() == 2 {
            return Ok(StringIndex::from(self.read_u16()?));
        }
        else {
            return Ok(StringIndex::from(self.read_u32()?));
        }
    }

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If e is an index into the GUID heap, 'blob', or String heap, it is stored using the number of bytes as defined in the HeapSizes field.
    pub fn read_blob_index(&mut self) -> Result<BlobIndex, std::io::Error> {
        if self.heap_sizes.blob_size() == 2 {
            return Ok(BlobIndex::from(self.read_u16()?));
        }
        else {
            return Ok(BlobIndex::from(self.read_u32()?));
        }
    }

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If e is an index into the GUID heap, 'blob', or String heap, it is stored using the number of bytes as defined in the HeapSizes field.
    pub fn read_guid_index(&mut self) -> Result<GuidIndex, std::io::Error> {
        if self.heap_sizes.guid_size() == 2 {
            return Ok(GuidIndex::from(self.read_u16()?));
        }
        else {
            return Ok(GuidIndex::from(self.read_u32()?));
        }
    }

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If e is a simple index into a table with index i, it is stored using 2 bytes if table i has less than 2<sup>16</sup> rows, otherwise it is stored using 4 bytes. 
    pub fn read_table_index(&mut self, kind: TableKind) -> Result<CodedIndex, std::io::Error> {
        let row_count = self.row_count.get(&kind).unwrap_or(&0);
        if *row_count < 0x10000 {
            return Ok(CodedIndex::from(kind, self.read_u16()? as u32));
        }
        else {
            return Ok(CodedIndex::from(kind, self.read_u32()?));
        }
    }
    
    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If *e* is a *coded index* that points into table *t<sub>i</sub>* out of *n* possible tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, then it 
    ///   is stored as e << (log n) | tag{*t<sub>0</sub>*, ...*t<sub>n-1</sub>*}[*t<sub>i</sub>] using 2 bytes if the maximum number 
    ///   of rows of tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, is less than 2<sup>(16 – (log n))</sup>, and using 4 bytes otherwise.
    ///   The family of finite maps tag{*t<sub>0</sub>, ...*t<sub>n-1</sub>*} is defined below. Note that decoding a physical 
    ///   row requires the inverse of this mapping. [For example, the Parent column of the 
    ///   *Constant* table indexes a row in the *Field*, *Param*, or *Property* tables.  The actual 
    ///   table is encoded into the low 2 bits of the number, using the values: 0 => *Field*, 1 => 
    ///   *Param*, 2 => *Property*. The remaining bits hold the actual row number being 
    ///   indexed. For example, a value of `0x321`, indexes row number `0xC8` in the *Param* table.]
    pub fn read_coded_index(&mut self, tag: CodedIndexTag) -> Result<CodedIndex, std::io::Error> {
        let index: u32 = if tag.is_big_index(|kind| *self.row_count.get(&kind).unwrap_or(&0)) {
            self.read_u32()?
        } else {
            self.read_u16()? as u32
        };

        let data = index >> tag.get_tag_size();
        let table = tag.get_table_kind((index & 0xff) as u8);
        Ok(CodedIndex::from(table, data))
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), std::io::Error> {
        self.buffer.read_exact(buffer)
    }

    pub fn read_until(&mut self, byte: u8, buffer: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        self.buffer.read_until(byte, buffer)
    }

    pub fn set_row_count(&mut self, kind: TableKind, count: u32) {
        self.row_count.insert(kind, count);
    }

    pub fn set_heap_sizes(&mut self, sizes: HeapSizes) {
        self.heap_sizes = sizes;
    }
}
