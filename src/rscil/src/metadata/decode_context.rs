use std::collections::HashMap;

use super::*;

pub struct TableDecodeContext {
    row_count: HashMap<TableKind, u32>,
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
        }
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

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If e is a simple index into a table with index i, it is stored using 2 bytes if table i has less than 2<sup>16</sup> rows, otherwise it is stored using 4 bytes. 
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
