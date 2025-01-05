use std::collections::HashMap;

use crate::{cast_row, AssemblyRow, ModuleRow, StringIndex};

use super::*;

macro_rules! define_getter {
    ($name:ident, $table:ident, $row:ident) => {
        pub fn $name(&self, index: u32) -> Option<$row> {
            cast_row!(Some(Row::$table), self.streams.metadata.get_table(TableKind::$table).get(index as usize))
        }
    };
}

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

    /// # II.22.2 Assembly : 0x20
    /// [...]
    /// 
    /// 1. The Assembly table shall contain zero or one row [ERROR]
    pub fn get_assembly(&self) -> Option<AssemblyRow> {
        cast_row!(Some(Row::Assembly), self.streams.metadata.get_table(TableKind::Assembly).get(0))
    }

    /// II.22.30 Module : 0x00
    /// [...]
    /// 
    /// 1. The Module table shall contain one and only one row [ERROR] 
    pub fn get_module(&self) -> &ModuleRow {
        cast_row!(Row::Module, self.streams.metadata.get_table(TableKind::Module).get(0).unwrap())
    }

    define_getter!(get_method_def, MethodDef, MethodDefRow);
    define_getter!(get_type_def, TypeDef, TypeDefRow);
    define_getter!(get_type_ref, TypeRef, TypeRefRow);
    define_getter!(get_field, Field, FieldRow);
    define_getter!(get_param, Param, ParamRow);
    define_getter!(get_interface_impl, InterfaceImpl, InterfaceImplRow);
    define_getter!(get_member_ref, MemberRef, MemberRefRow);
    define_getter!(get_assembly_ref, AssemblyRef, AssemblyRefRow);

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