use std::collections::HashMap;

use crate::{cast_row, StringIndex};

use super::*;

// # [II.22] Metadata logical format : tables
//
// [...]
//
// Indexes to tables begin at 1, so index 1 means the first row in any given metadata table.  (An index 
// value of zero denotes that it does not index a row at all; that is, it behaves like a null reference.) 
//
// [II.22]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=235
macro_rules! define_getter {
    ($name:ident, $row:ident) => {
        pub fn $name(&self, index: u32) -> Option<&$row> {
            if (index == 0) {
                return None;
            }
            self.streams.metadata.get_table(TableKind::$row).get((index - 1) as usize).map(|row| cast_row!(Row::$row, row))
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
    pub fn new(filename: String, cli_header: CliHeader, metadata_header: MetadataHeader, streams: Streams, mut buffer: PeParser) -> PeImage {

        let methods = Self::construct_method_body_map(streams.metadata.get_table(TableKind::MethodDef), &mut buffer);

        PeImage {
            filename,
            cli_header,
            metadata_header,
            streams,
            buffer,
            methods,
        }
    }

    fn construct_method_body_map(methods: &Table, buffer: &mut PeParser) -> HashMap<u32, MethodBody> {
        let mut map = HashMap::new();
        for (_, row) in methods.iter().enumerate() {
            if let Some(method) = MethodDef::from_row(row) {
                let body = buffer.read_method_body(method.rva).unwrap();
                map.insert(method.index, body);
            }
        }
        map
    }

    pub fn get_string(&self, index: StringIndex) -> &String {
        self.streams.strings.get(index.0).unwrap()
    }

    /// # [II.22.2] Assembly : 0x20
    /// [...]
    /// 
    /// 1. The Assembly table shall contain zero or one row [ERROR]
    /// 
    /// [II.22.2]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=237
    pub fn get_assembly(&self) -> Option<&Assembly> {
        self.streams.metadata.get_table(TableKind::Assembly).first().map(|row| cast_row!(Row::Assembly, row))
    }

    /// # [II.22.30] Module : 0x00
    /// [...]
    /// 
    /// 1. The Module table shall contain one and only one row [ERROR] 
    /// 
    /// [II.22.30]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=265
    pub fn get_module(&self) -> &Module {
        cast_row!(Some(Module), self.streams.metadata.get_table(TableKind::Module).first()).unwrap()
    }

    define_getter!(get_method_def, MethodDef);
    define_getter!(get_type_def, TypeDef);
    define_getter!(get_type_ref, TypeRef);
    define_getter!(get_field, Field);
    define_getter!(get_param, Param);
    define_getter!(get_interface_impl, InterfaceImpl);
    define_getter!(get_member_ref, MemberRef);
    define_getter!(get_assembly_ref, AssemblyRef);

    pub fn get_method_body(&self, method_index: u32) -> Option<&MethodBody> {
        self.methods.get(&method_index)
    }
}
