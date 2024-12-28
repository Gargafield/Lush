
use super::PeParser;

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
    pub fn from(buffer: &mut PeParser) -> Result<MetadataHeader, std::io::Error> {
        let signature = buffer.read_u32()?;

        // See Description of Signature field in the table above
        assert!(signature == 0x424A5342, "Invalid metadata signature: 0x{:X}", signature);

        let major_version = buffer.read_u16()?;
        let minor_version = buffer.read_u16()?;
        let reserved = buffer.read_u32()?;
        let length = buffer.read_u32()?;

        let mut version = vec![0u8; length as usize];
        buffer.read_exact(&mut version)?;
        let version = String::from_utf8(version).unwrap();

        let mut padding = vec![0u8; (length % 4) as usize];
        buffer.read_exact(&mut padding)?;

        let flags = buffer.read_u16()?;
        let streams = buffer.read_u16()?;

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
    pub fn from(buffer: &mut PeParser) -> Result<StreamHeader, std::io::Error> {
        let offset = buffer.read_u32()?;
        let size = buffer.read_u32()?;

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