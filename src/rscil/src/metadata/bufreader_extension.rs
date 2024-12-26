use std::{fs::File, io::{BufReader, Read}};

pub(in super) trait BufReaderExtension {
    /// Reads a single byte from the buffer.
    fn read_u8(&mut self) -> Result<u8, std::io::Error>;

    /// Reads a u16 from the buffer.
    fn read_u16(&mut self) -> Result<u16, std::io::Error>;

    /// Reads a u32 from the buffer.
    fn read_u32(&mut self) -> Result<u32, std::io::Error>;

    /// Reads a u64 from the buffer.
    fn read_u64(&mut self) -> Result<u64, std::io::Error>;
}

impl BufReaderExtension for BufReader<File> {
    fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_u16(&mut self) -> Result<u16, std::io::Error> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    fn read_u32(&mut self) -> Result<u32, std::io::Error> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    fn read_u64(&mut self) -> Result<u64, std::io::Error> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}