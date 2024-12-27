use crate::{cast_table, AssemblyRow, ModuleRow, StringIndex};

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
        self.streams.strings.get(index.0).unwrap()
    }

    pub fn get_assembly(&self) -> Option<AssemblyRow> {
        cast_table!(Assembly, self.streams.metadata.get_table(TableKind::Assembly))
    }

    pub fn get_module(&self) -> ModuleRow {
        cast_table!(Module, self.streams.metadata.get_table(TableKind::Module))
    }
}