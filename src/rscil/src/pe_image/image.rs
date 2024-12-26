use crate::{AssemblyRow, AssemblyTable, ModuleRow, ModuleTable, StringIndex};

use super::*;

pub struct PeImage {
    pub filename : String,
    pub pe_header: PeHeader,
    pub optional_header: PeOptionalHeader,
    pub sections: Vec<SectionHeader>,
    pub cli_header: CliHeader,
    pub metadata_header: MetadataHeader,
    pub streams: Streams,
}

impl PeImage {
    pub fn get_string(&self, index: StringIndex) -> &String {
        self.streams.strings.get(index.0 as u32).unwrap()
    }

    pub fn get_assembly(&self) -> Option<AssemblyRow> {
        self.streams.metadata.get_table::<AssemblyTable>(crate::TableKind::Assembly).unwrap().row
    }

    pub fn get_module(&self) -> ModuleRow {
        self.streams.metadata.get_table::<ModuleTable>(crate::TableKind::Module).unwrap().0
    }
}