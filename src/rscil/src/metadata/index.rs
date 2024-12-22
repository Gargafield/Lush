use std::io::Read;



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringIndex(pub u16);

impl StringIndex {
    pub fn read(buffer: &mut dyn Read) -> Result<StringIndex, std::io::Error> {
        let mut index = [0u8; 2];
        buffer.read_exact(&mut index)?;
        Ok(StringIndex(u16::from_le_bytes(index)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GuidIndex(pub u16);
impl GuidIndex {
    pub fn read(buffer: &mut dyn Read) -> Result<GuidIndex, std::io::Error> {
        let mut index = [0u8; 2];
        buffer.read_exact(&mut index)?;
        Ok(GuidIndex(u16::from_le_bytes(index)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlobIndex(pub u16);
impl BlobIndex {
    pub fn read(buffer: &mut dyn Read) -> Result<BlobIndex, std::io::Error> {
        let mut index = [0u8; 2];
        buffer.read_exact(&mut index)?;
        Ok(BlobIndex(u16::from_le_bytes(index)))
    }
}
