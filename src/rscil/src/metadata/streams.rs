use std::{any::Any, collections::HashMap, fs::File, io::{BufRead, BufReader, Read, Seek}};

use crate::*;

/// II.24.2.2 Stream header 
/// 
/// [...]
/// There are five possible kinds of streams. A stream 
/// header with name "#Strings" that points to the physical representation of the string heap where 
/// identifier strings are stored; a stream header with name "#US" that points to the physical representation 
/// of the user string heap; a stream header with name "#Blob" that points to the physical representation of 
/// the blob heap, a stream header with name "#GUID" that points to the physical representation of the 
/// GUID heap; and a stream header with name "#~" that points to the physical representation of a set of 
/// tables. 
pub struct Streams {
    pub strings: StringStream,
    pub user_strings: UserStringStream,
    pub blobs: BlobStream,
    // TODO: GUIDStream
    pub metadata: MetadataStream,
}

impl Streams {
    pub fn from(buffer: &mut BufReader<File>, root_address: u64, headers: &Vec<StreamHeader>) -> Result<Streams, std::io::Error> {
        let mut strings = None;
        let mut user_strings = None;
        let mut blobs = None;
        let mut metadata = None;

        for header in headers {
            buffer.seek(std::io::SeekFrom::Start(root_address + header.offset as u64))?;
            match header.name.as_str() {
                "#Strings" => strings = Some(StringStream::from(buffer, header)?),
                "#US" => user_strings = Some(UserStringStream::from(buffer, header)?),
                "#Blob" =>blobs = Some(BlobStream::from(buffer, header)?),
                "#~" => metadata = Some(MetadataStream::from(buffer)?),
                _ => (),
            }
        }

        Ok(Streams {
            strings: strings.unwrap(),
            user_strings: user_strings.unwrap(),
            blobs: blobs.unwrap(),
            metadata: metadata.unwrap(),
        })
    }
}



/// # II.24.2.3 #Strings heap 
/// 
/// The stream of bytes pointed to by a “#Strings” header is the physical representation of the logical string 
/// heap. The physical heap can contain garbage, that is, it can contain parts that are unreachable from any 
/// of the tables, but parts that are reachable from a table shall contain a valid null-terminated UTF8 string. 
/// When the #String heap is present, the first entry is always the empty string (i.e., \0).
///
/// # II.22 Metadata logical format: tables 
/// 
/// [...]
/// 
/// Metadata is stored in two kinds of structure: tables (arrays of records) and heaps. There are four heaps 
/// in any module: String, Blob, Userstring, and Guid.  The first three are byte arrays (so valid indexes into 
/// these heaps might be 0, 23, 25, 39, etc). The Guid heap is an array of GUIDs, each 16 bytes wide. Its 
/// first element is numbered 1, its second 2, and so on. 
pub struct StringStream(HashMap<u32, String>);

impl StringStream {
    pub fn from(buffer: &mut BufReader<File>, header: &StreamHeader) -> Result<StringStream, std::io::Error> {
        let mut strings = HashMap::new();
        let mut count = 0;
        while count < header.size {
            let mut string = Vec::new();
            count += buffer.read_until(0, &mut string)? as u32;
            let string = String::from_utf8(string).unwrap();
            strings.insert(count, string);
        }
        Ok(StringStream(strings))
    }

    pub fn get(&self, index: u32) -> Option<&String> {
        self.0.get(&index)
    }
}

/// # II.24.2.4 #US and #Blob heaps
/// 
/// The stream of bytes pointed to by a "#US" or "#Blob" header are the physical representation of logical 
/// Userstring and 'blob' heaps respectively. Both these heaps can contain garbage, as long as any part that 
/// is reachable from any of the tables contains a valid 'blob'. Individual blobs are stored with their length 
/// encoded in the first few bytes:
/// 
/// See [BlobStream::read_length].
/// 
/// The first entry in both these heaps is the empty 'blob' that consists of the single byte 0x00. 
///
/// # II.22 Metadata logical format: tables 
/// 
/// See [`StringStream`].
pub struct BlobStream(pub HashMap<u32, Vec<u8>>);

impl BlobStream {
    pub fn from(buffer: &mut BufReader<File>, header: &StreamHeader) -> Result<BlobStream, std::io::Error> {
        let mut blobs = HashMap::new();
        let mut count = 0;
        while count < header.size {
            let (length, bytes_read) = read_blob_length(buffer)?;

            let mut blob = vec![0u8; length as usize];
            buffer.read_exact(&mut blob).unwrap();
            blobs.insert(count, blob);
            count += bytes_read as u32;
            count += length as u32;
        }
        Ok(BlobStream(blobs))
    }
}

/// # II.24.2.4 #US and #Blob heaps
/// 
/// [...]
/// 
/// * If the first one byte of the 'blob' is *0bbbbbbb<sub>2</sub>*, then the rest of the 'blob' contains the *bbbbbbb<sub>2</sub>* bytes of actual data.
/// * If the first two bytes of the 'blob' are *10bbbbbb<sub>2</sub>* and *x*, then the rest of the 'blob' contains the (*bbbbbb<sub>2</sub>* << 8 + *x*) bytes of actual data. 
/// * If the first four bytes of the 'blob' are *110bbbbb<sub>2</sub>*, *x*, *y*, and *z*, then the rest of the 'blob' contains the (*bbbbb<sub>2</sub>* << 24 + *x* << 16 + *y* << 8 + *z*) bytes of actual data. 
///
/// Returns the length of the 'blob' and the number of bytes read from the buffer.
fn read_blob_length(buffer: &mut BufReader<File>) -> Result<(usize, u32), std::io::Error> {
    let first = buffer.read_u8()?;

    if first & 0b1000_0000 == 0 {
        Ok((first as usize, 1))
    } else if first & 0b1100_0000 == 0b1000_0000 {
        let length = ((first & 0b0011_1111) as u32) << 8 + buffer.read_u8()? as u32;
        Ok((length as usize, 2))
    } else if first & 0b1110_0000 == 0b1100_0000 {
        let mut bytes = [0u8; 3];
        buffer.read_exact(&mut bytes).unwrap();
        let length = ((first & 0b0001_1111) as u32) << 24
            + (bytes[0] as u32) << 16
            + (bytes[1] as u32) << 8
            + (bytes[2] as u32);
        Ok((length as usize, 4))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid blob length"))
    }
}

/// # II.24.2.4 #US and #Blob heaps
/// 
/// See [`BlobStream`].
/// 
/// Strings in the #US (user string) heap are encoded using 16-bit Unicode encodings. The count on each 
/// string is the number of bytes (not characters) in the string. Furthermore, there is an additional terminal 
/// byte (so all byte counts are odd, not even). This final byte holds the value 1 if and only if any UTF16 
/// character within the string has any bit set in its top byte, or its low byte is any of the following: `0x01`
/// `0x08`, `0x0E`–`0x1F`, `0x27`, `0x2D`, `0x7F`.  Otherwise, it holds 0. The 1 signifies Unicode characters that 
/// require handling beyond that normally provided for 8-bit encoding sets.
///
/// # II.22 Metadata logical format: tables 
/// 
/// See [`StringStream`].
pub struct UserStringStream(pub HashMap<u32, Vec<u16>>);

impl UserStringStream {
    pub fn from(buffer: &mut BufReader<File>, header: &StreamHeader) -> Result<UserStringStream, std::io::Error> {
        let mut strings = HashMap::new();
        let mut count = 0;
        while count < header.size {
            let (length, bytes_read) = read_blob_length(buffer)?;

            // Read UTF-16 string
            let mut string = vec![0u16; length / 2];
            for i in 0..(length / 2) {
                string[i] = buffer.read_u16()?;
            }

            strings.insert(count, string);
            count += bytes_read as u32;
            count += length as u32;
        }
        Ok(UserStringStream(strings))
    }
}

// TODO: Implement GUIDStream

/// # II.24.2.6 #~ stream 
/// 
/// The "#~" streams contain the actual physical representations of the logical metadata tables (§II.22).
/// A "#~" stream has the following top-level structure:
/// 
/// | Offset   | Size   | Field        | Description |
/// | -------- | ------ | ------------ | ----------- |
/// | 0        | 4      | Reserved     | Reserved, always 0 (§II.24.1) |
/// | 4        | 1      | MajorVersion | Major version of table schemata; shall be 2 (§II.24.1) |
/// | 5        | 1      | MinorVersion | Minor version of table schemata; shall be 0 (§II.24.1) 
/// | 6        | 1      | HeapSizes    | Bit vector for heap sizes. |
/// | 7        | 1      | Reserved     | Reserved, always 1 (§II.24.1) |
/// | 8        | 8      | Valid        | Bit vector of present tables, let n be the number of bits that are 1. |
/// | 16       | 8      | Sorted       | Bit vector of sorted tables. |
/// | 24       | 4**n*  | Rows         | Array of n 4-byte unsigned integers indicating the number of rows for each present table. |
/// | 24+4**n* |        | Tables       | The sequence of physical tables. |
/// 
/// The Valid field is a 64-bit bitvector that has a specific bit set for each table that is stored in the stream; 
/// the mapping of tables to indexes is given at the start of §II.22. For example when the `DeclSecurity` 
/// table is present in the logical metadata, bit `0x0e` should be set in the Valid vector. It is invalid to 
/// include non-existent tables in Valid, so all bits above `0x2c` shall be zero. 
///
/// The Rows array contains the number of rows for each of the tables that are present. When decoding 
/// physical metadata to logical metadata, the number of 1's in Valid indicates the number of elements in 
/// the Rows array.
/// 
/// [...]
pub struct MetadataStream {
    pub major_version: u8,
    pub minor_version: u8,
    pub heap_sizes: HeapSizes,
    pub valid: u64,
    pub sorted: u64,
    pub rows: Vec<u32>,
    pub tables: HashMap<TableKind, Box<dyn Any>>,
}

impl MetadataStream {
    pub fn from(buffer: &mut BufReader<File>) -> Result<MetadataStream, std::io::Error> {
        buffer.read_u32()?; // Reserved

        let major_version = buffer.read_u8()?;
        let minor_version = buffer.read_u8()?;
        assert_eq!(major_version, 2, "Invalid major version");
        assert_eq!(minor_version, 0, "Invalid minor version");
        
        let heap_sizes = HeapSizes::from(buffer.read_u8()?);
        buffer.read_u8()?; // Reserved
        let valid = buffer.read_u64()?;
        let sorted = buffer.read_u64()?;

        let mut rows = Vec::new();
        let number_of_tables = valid.count_ones();
        for _ in 0..number_of_tables {
            rows.push(buffer.read_u32()?);
        }

        let mut tables = HashMap::<TableKind, Box<dyn Any>>::new();

        let table_kinds = TableKind::from_bitmask(valid);
        for (i, kind) in table_kinds.iter().enumerate() {
            tables.insert(*kind, match kind {
                TableKind::Assembly => Box::new(AssemblyTable::read_from(buffer, rows[i])?),
                TableKind::AssemblyRef => Box::new(AssemblyRefTable::read_from(buffer, rows[i])?),
                TableKind::Constant => Box::new(ConstantTable::read_from(buffer, rows[i])?),
                TableKind::CustomAttribute => Box::new(CustomAttributeTable::read_from(buffer, rows[i])?),
                TableKind::Field => Box::new(FieldTable::read_from(buffer, rows[i])?),
                TableKind::MemberRef => Box::new(MemberRefTable::read_from(buffer, rows[i])?),
                TableKind::MethodDef => Box::new(MethodDefTable::read_from(buffer, rows[i])?),
                TableKind::Module => Box::new(ModuleTable::read_from(buffer, rows[i])?),
                TableKind::Param => Box::new(ParamTable::read_from(buffer, rows[i])?),
                TableKind::TypeDef => Box::new(TypeDefTable::read_from(buffer, rows[i])?),
                TableKind::TypeRef => Box::new(TypeRefTable::read_from(buffer, rows[i])?),
                _ => panic!("Unknown table kind: {:?}", kind),
            });
        }

        Ok(MetadataStream {
            major_version,
            minor_version,
            heap_sizes,
            valid,
            sorted,
            rows,
            tables,
        })
    }

    pub fn get_table<T: Any + 'static>(&self, kind: TableKind) -> Option<&T> {
        self.tables.get(&kind).and_then(|table| table.downcast_ref::<T>())
    }
}

/// # II.24.2.6 #~ stream 
/// [...]
/// 
/// The HeapSizes field is a bitvector that encodes the width of indexes into the various heaps.  If bit 0 is 
/// set, indexes into the “#String” heap are 4 bytes wide; if bit 1 is set, indexes into the “#GUID” heap are 
/// 4 bytes wide; if bit 2 is set, indexes into the “#Blob” heap are 4 bytes wide.  Conversely, if the 
/// HeapSize bit for a particular heap is not set, indexes into that heap are 2 bytes wide. 
/// | Heap size flag | Description | 
/// | -------------- | ----------- |
/// | `0x01`         | Size of "#String" stream >= 216. |
/// | `0x02`         | Size of "#GUID" stream >= 216. |
/// | `0x04`         | Size of "#Blob" stream >= 216. |
pub struct HeapSizes(u8);

impl HeapSizes {
    pub const STRING_FLAG: u8 = 0b0000_0001;
    pub const GUID_FLAG: u8 = 0b0000_0010;
    pub const BLOB_FLAG: u8 = 0b0000_0100;

    pub fn from(value: u8) -> HeapSizes {
        HeapSizes(value)
    }

    pub fn string_size(&self) -> usize {
        if self.check_flag(HeapSizes::STRING_FLAG) {
            4
        } else {
            2
        }
    }

    pub fn guid_size(&self) -> usize {
        if self.check_flag(HeapSizes::GUID_FLAG) {
            4
        } else {
            2
        }
    }

    pub fn blob_size(&self) -> usize {
        if self.check_flag(HeapSizes::BLOB_FLAG) {
            4
        } else {
            2
        }
    }

    pub fn check_flag(&self, flag: u8) -> bool {
        self.0 & flag == flag
    }
    
}