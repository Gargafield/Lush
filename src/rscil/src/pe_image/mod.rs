pub mod characteristics;
pub mod pe_header;
pub mod pe_optional_header; 
pub mod standard_fields;
pub mod nt_specific_fields;
pub mod data_directories;
pub mod section_header;
pub mod cli_header;

use std::{fs::File, io::{BufReader, Read, Seek, SeekFrom}};

use pe_header::PeHeader;
use pe_optional_header::PeOptionalHeader;

pub struct PeImage {
    filename : String,
    buffer : BufReader<File>,
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

impl PeImage {
    pub fn new(filename: &str, buffer: BufReader<File>) -> PeImage {
        PeImage {
            filename: filename.to_string(),
            buffer: buffer,
        }
    }

    pub fn open(filename: &str) -> Result<PeImage, std::io::Error> {
        let file = File::open(filename)?;
        let buffer = BufReader::new(file);
        Ok(PeImage::new(filename, buffer))
    }    

    /// # II.25.2.1 MS-DOS header
    /// The PE format starts with an MS-DOS stub of exactly the following **128** bytes to be placed at the front 
    /// of the module. At offset `0x3c` in the DOS header is a 4-byte unsigned integer offset, `lfanew`, to the PE 
    /// signature (shall be "PE\0\0"), immediately followed by the PE file header.
    fn read_dos_stub(&mut self) -> Result<(), std::io::Error> {
        let mut header = [0u8; 128];
        self.buffer.read_exact(&mut header)?;

        if header[..0x3c] != DOS_STUB[..0x3c] || header[0x40..] != DOS_STUB[0x40..] {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid DOS header"));
        }

        let lfanew = u32::from_le_bytes(header[0x3c..0x40].try_into().unwrap());

        // Check if the PE signature is present
        let mut signature = [0u8; 4];
        self.buffer.read_exact(&mut signature)?;
        self.buffer.seek(SeekFrom::Start(lfanew as u64))?;
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
}



