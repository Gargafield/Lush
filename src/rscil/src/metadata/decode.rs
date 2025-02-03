
pub(crate) type Buffer = Cursor<Vec<u8>>;

use std::{cell::RefCell, collections::HashMap};

use super::*;

pub struct TableDecodeContext {
    row_count: HashMap<TableKind, u32>,
    index_tracker: RefCell<HashMap<TableKind, u32>>,
    pub heap_sizes: HeapSizes,
    coded_index_sizes: HashMap<CodedIndexTag, u8>,
}

impl TableDecodeContext {
    
    #[must_use]
    pub fn new(row_count: HashMap<TableKind, u32>, heap_sizes: HeapSizes) -> Self {
        let coded_index_sizes = Self::compute_coded_index_sizes(&row_count);

        Self {
            row_count,
            heap_sizes,
            coded_index_sizes,
            index_tracker: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_index(&self, kind: TableKind) -> u32 {
        let mut index_tracker = self.index_tracker.borrow_mut();
        let index = *index_tracker.get(&kind).unwrap_or(&1);
        index_tracker.insert(kind, index + 1);
        index
    }

    fn compute_coded_index_sizes(row_count: &HashMap<TableKind, u32>) -> HashMap<CodedIndexTag, u8> {
        let mut coded_index_sizes = HashMap::<CodedIndexTag, u8>::new();

        for (tag, _) in CodedIndexTag::iter() {
            let size = if tag.is_big_index(|kind| *row_count.get(&kind).unwrap_or(&0)) {
                4
            } else {
                2
            };
            coded_index_sizes.insert(*tag, size);
        }

        coded_index_sizes
    }

    /// # [II.24.2.6] #~ stream 
    /// 
    /// [...]
    /// 
    /// * If e is a simple index into a table with index i, it is stored using 2 bytes if table i has less than 2<sup>16</sup> rows, otherwise it is stored using 4 bytes. 
    /// 
    /// [II.24.2.6]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=299
    pub fn get_table_index_size(&self, kind: TableKind) -> u8 {
        let row_count = self.row_count.get(&kind).unwrap_or(&0);
        if *row_count < 0x10000 {
            2
        }
        else {
            4
        }
    }

    pub fn get_row_count(&self, kind: TableKind) -> u32 {
        *self.row_count.get(&kind).unwrap_or(&0)
    }

    pub fn get_coded_index_size(&self, tag: CodedIndexTag) -> u8 {
        *self.coded_index_sizes.get(&tag).unwrap_or(&0)
    }
}

pub trait TableDecode : Sized {
    type Output;

    fn decode(context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self::Output, std::io::Error>;
}

pub trait TableEnumDecode : Sized {
    type Output;

    fn decode(self, context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self::Output, std::io::Error>;
}

impl TableDecode for u32 {
    type Output = Self;
    fn decode(_context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self, std::io::Error> {
        buffer.read_u32::<LittleEndian>()
    }
}

impl TableDecode for u16 {
    type Output = Self;
    fn decode(_context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self, std::io::Error> {
        buffer.read_u16::<LittleEndian>()
    }
}

impl TableDecode for u8 {
    type Output = Self;
    fn decode(_context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self, std::io::Error> {
        buffer.read_u8()
    }
}

