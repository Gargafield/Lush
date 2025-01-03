use std::collections::HashMap;

use crate::{cast_table, AssemblyRow, ModuleRow, StringIndex};

use super::*;

pub struct PeImage {
    pub filename : String,
    pub cli_header: CliHeader,
    pub metadata_header: MetadataHeader,
    pub streams: Streams,
    pub buffer: PeParser,

    methods: HashMap<u32, MethodBody>,
}

impl PeImage {
    pub fn new(filename: String, cli_header: CliHeader, metadata_header: MetadataHeader, streams: Streams, buffer: PeParser) -> PeImage {
        PeImage {
            filename: filename,
            cli_header,
            metadata_header,
            streams,
            buffer,
            methods: HashMap::new(),
        }
    }


    pub fn get_string(&self, index: StringIndex) -> &String {
        self.streams.strings.get(index.0).unwrap()
    }

    pub fn get_assembly(&self) -> &Option<AssemblyRow> {
        cast_table!(Assembly, self.streams.metadata.get_table(TableKind::Assembly))
    }

    pub fn get_module(&self) -> &ModuleRow {
        cast_table!(Module, self.streams.metadata.get_table(TableKind::Module))
    }

    pub fn get_method_def(&self, index: u32) -> Option<MethodDefRow> {
        let table = cast_table!(MethodDef, self.streams.metadata.get_table(TableKind::MethodDef));
        table.get(index as usize).map(|x| *x)
    }

    pub fn get_method_body(&mut self, method_index: u32) -> Result<&MethodBody, std::io::Error> {
        if self.methods.contains_key(&method_index) {
            return Ok(self.methods.get(&method_index).unwrap());
        }
        else {
            let method = self.get_method_def(method_index).unwrap();    
            let body = self.buffer.read_method_body(method.rva)?;
            self.methods.insert(method_index, body);
            return Ok(self.methods.get(&method_index).unwrap());
        }
    }
}