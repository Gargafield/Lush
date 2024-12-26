use std::{fs::File, io::{BufReader, Read}};

use super::bufreader_extension::BufReaderExtension;

pub trait Index {
    fn read(buffer: &mut BufReader<File>) -> Result<Self, std::io::Error> where Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringIndex(pub u32);

impl Index for StringIndex {
    fn read(buffer: &mut BufReader<File>) -> Result<StringIndex, std::io::Error> {
        Ok(StringIndex(buffer.read_u16()? as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GuidIndex(pub u32);
impl Index for GuidIndex {
    fn read(buffer: &mut BufReader<File>) -> Result<GuidIndex, std::io::Error> {
        Ok(GuidIndex(buffer.read_u16()? as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlobIndex(pub u32);
impl Index for BlobIndex {
    fn read(buffer: &mut BufReader<File>) -> Result<BlobIndex, std::io::Error> {
        Ok(BlobIndex(buffer.read_u16()? as u32))
    }
}

