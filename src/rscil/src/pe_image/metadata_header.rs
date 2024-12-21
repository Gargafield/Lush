use std::{fs::File, io::{BufRead, BufReader, Read}};

/// # II.24.2 File headers
/// ## II.24.2.1 Metadata root
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
/// | 8            | 4        | Reserved      | Reserved, always 0 (§II.24.1). |
/// | 12           | 4        | Length        | Number of bytes allocated to hold version string (including null terminator), call this *x*. Call the length of the string (including the terminator) *m* (we require *m* <= 255); the length *x* is *m* rounded up to a multiple of four. |
/// | 16           | *m*      | Version       | UTF8-encoded null-terminated version string of length *m* (see above) |
/// | 16+*m*       | *x*-*m*  | Padding       | Padding to next 4 byte boundary. |
/// | 16+*x*       | 2        | Flags         | Reserved, always 0 (§II.24.1). |
/// | 16+*x*+2     | 2        | Streams       | Number of streams, say n. |
/// | 16+*x*+4     | -        | StreamHeaders | Array of n StreamHdr structures. |
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
    pub fn from(buffer: &mut BufReader<File>) -> Result<MetadataHeader, std::io::Error> {
        let mut header = [0u8; 16];
        buffer.read_exact(&mut header)?;

        let signature = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let major_version = u16::from_le_bytes(header[4..6].try_into().unwrap());
        let minor_version = u16::from_le_bytes(header[6..8].try_into().unwrap());
        let reserved = u32::from_le_bytes(header[8..12].try_into().unwrap());
        let length = u32::from_le_bytes(header[12..16].try_into().unwrap());

        let mut version = vec![0u8; length as usize];
        buffer.read_exact(&mut version)?;
        let version = String::from_utf8(version).unwrap();

        let mut padding = vec![0u8; (length % 4) as usize];
        buffer.read_exact(&mut padding)?;

        let mut flags = [0u8; 2];
        buffer.read_exact(&mut flags)?;
        let flags = u16::from_le_bytes(flags);

        let mut streams = [0u8; 2];
        buffer.read_exact(&mut streams)?;
        let streams = u16::from_le_bytes(streams);

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

/// # II.24.2.2 Stream header 
/// A stream header gives the names, and the position and length of a particular table or heap. Note that the 
/// length of a Stream header structure is not fixed, but depends on the length of its name field (a variable 
/// length null-terminated string).
/// 
/// | Offset | Size | Field  | Description |
/// | ------ | ---- | ------ | ----------- |
/// | 0      | 4    | Offset | Memory offset to start of this stream from start of the metadata root (§II.24.2.1) | 
/// | 4      | 4    | Size   | Size of this stream in bytes, shall be a multiple of 4. |
/// | 8      | -    | Name   | Name of the stream as null-terminated variable length array of ASCII characters, padded to the next 4-byte boundary with `\0` characters. The name is limited to 32 characters. |
pub struct StreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}

impl StreamHeader {
    pub fn from(buffer: &mut BufReader<File>) -> Result<StreamHeader, std::io::Error> {
        let mut header = [0u8; 8];
        buffer.read_exact(&mut header)?;

        let offset = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let size = u32::from_le_bytes(header[4..8].try_into().unwrap());

        let mut name = Vec::new();
        buffer.read_until(0, &mut name)?;
        let name = String::from_utf8(name).unwrap();

        let padding = (4 - (name.len() % 4)) % 4;
        let mut padding = vec![0u8; padding];
        buffer.read_exact(&mut padding)?;

        Ok(StreamHeader {
            offset,
            size,
            name,
        })
    }
}