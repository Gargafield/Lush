
use std::slice::Iter;

use super::*;

macro_rules! define_stream_index {
    ($name:ident, $flag:path) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name(pub u32);

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                $name(value)
            }
        }

        impl TableDecode for $name {
            type Output = $name;
            /// # II.24.2.6 #~ stream 
            /// 
            /// [...]
            /// 
            /// * If e is an index into the GUID heap, 'blob', or String heap, it is stored using the number of bytes as defined in the HeapSizes field.
            fn decode(context: &TableDecodeContext, buffer: &mut Buffer) -> Result<$name, std::io::Error> {
                if !context.heap_sizes.contains($flag) {
                    return Ok($name::from(buffer.read_u16::<LittleEndian>()?));
                }
                else {
                    return Ok($name::from(buffer.read_u32::<LittleEndian>()?));
                }
            }
        }

        impl From<u16> for $name {
            fn from(value: u16) -> Self {
                $name(value as u32)
            }
        }

    };
}

define_stream_index!(StringIndex, HeapSizes::STRING_FLAG);
define_stream_index!(GuidIndex, HeapSizes::GUID_FLAG);
define_stream_index!(BlobIndex, HeapSizes::BLOB_FLAG);

macro_rules! define_coded_index_tag {
    {
        $(#[$meta:meta])*
        $vis:vis enum $name:ident : $name_num:tt {
            $(
                $(#[$collection_meta:meta])*
                $collection:ident : $bits_num:tt = [
                    $(
                        $tag:ident : $tag_num:tt,
                    )*
                ],
            )*
        }
    } => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$collection_meta])*
                $collection
            ),*
        }

        impl $name {
            pub fn get_tag_size(&self) -> u8 {
                match self {
                    $(
                        $name::$collection => $bits_num,
                    )*
                }
            }

            pub fn get_table_kind(&self, data: u8) -> TableKind {
                let data = data & ((1 << self.get_tag_size()) - 1);

                match self {
                    $(
                        $name::$collection => {
                            match data {
                                $(
                                    $tag_num => TableKind::$tag,
                                )*
                                // TODO: Fix
                                _ => TableKind::TypeDef,
                            }
                        },
                    )*
                }
            }

            /// # [II.24.2.6] #~ stream 
            /// 
            /// [...]
            /// 
            /// * If *e* is a *coded index* that points into table *t<sub>i</sub>* out of *n* possible tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, then it 
            ///   is stored as e << (log n) | tag{*t<sub>0</sub>*, ...*t<sub>n-1</sub>*}\[*t<sub>i</sub>*] using 2 bytes if the maximum number 
            ///   of rows of tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, is less than 2<sup>(16 – (log n))</sup>, and using 4 bytes otherwise.
            /// 
            /// [II.24.2.6]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=299
            pub fn is_big_index(&self, row_count: impl Fn(TableKind) -> u32) -> bool {
                match self {
                    $(
                        $name::$collection => {
                            let max = [$(
                                row_count(TableKind::$tag),
                            )*];
                            *max.iter().max().unwrap() > 2u32.pow(16 - $bits_num)
                        },
                    )*
                }
            }

            pub fn iter() -> Iter<'static, ($name, u8)> {
                static ITER: [($name, u8); $name_num] = [
                    $(
                        ($name::$collection, $bits_num),
                    )*
                ];
                ITER.iter()
            }
        }
    };
}

define_coded_index_tag!{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CodedIndexTag : 13 {
        /// | TypeDefOrRef: 2 bits to encode tag | Tag |
        /// | ---------------------------------- | --- |
        /// | `TypeDef`                          | 0   |
        /// | `TypeRef`                          | 1   |
        /// | `TypeSpec`                         | 2   |
        TypeDefOrRef : 2 = [
            TypeDef : 0,
            TypeRef : 1,
            TypeSpec : 2,
        ],
        /// | HasConstant: 2 bits to encode tag | Tag |
        /// | --------------------------------- | --- |
        /// | `Field`                           | 0   |
        /// | `Param`                           | 1   |
        /// | `Property`                        | 2   |
        HasConstant : 2 = [
            Field : 0,
            Param : 1,
            Property : 2,
        ],
        /// | HasCustomAttribute: 5 bits to encode tag | Tag |
        /// | ---------------------------------------- | --- |
        /// | `MethodDef`                              | 0   |
        /// | `Field`                                  | 1   |
        /// | `TypeRef`                                | 2   |
        /// | `TypeDef`                                | 3   |
        /// | `Param`                                  | 4   |
        /// | `InterfaceImpl`                          | 5   |
        /// | `MemberRef`                              | 6   |
        /// | `Module`                                 | 7   |
        /// | `Permission`                             | 8   |
        /// | `Property`                               | 9   |
        /// | `Event`                                  | 10  |
        /// | `StandAloneSig`                          | 11  |
        /// | `ModuleRef`                              | 12  |
        /// | `TypeSpec`                               | 13  |
        /// | `Assembly`                               | 14  |
        /// | `AssemblyRef`                            | 15  |
        /// | `File`                                   | 16  |
        /// | `ExportedType`                           | 17  |
        /// | `ManifestResource`                       | 18  |
        /// | `GenericParam`                           | 19  |
        /// | `GenericParamConstraint`                 | 20  |
        /// | `MethodSpec`                             | 21  |
        HasCustomAttribute : 5 = [
            MethodDef : 0,
            Field : 1,
            TypeRef : 2,
            TypeDef : 3,
            Param : 4,
            InterfaceImpl : 5,
            MemberRef : 6,
            Module : 7,
            Property : 9,
            Event : 10,
            StandAloneSig : 11,
            ModuleRef : 12,
            TypeSpec : 13,
            Assembly : 14,
            AssemblyRef : 15,
            File : 16,
            ExportedType : 17,
            ManifestResource : 18,
            GenericParam : 19,
            GenericParamConstraint : 20,
            MethodSpec : 21,
        ],
        /// | HasFieldMarshall: 1 bit to encode tag | Tag |
        /// | ------------------------------------- | --- |
        /// | `Field`                               | 0   |
        /// | `Param`                               | 1   |
        HasFieldMarshal : 1 = [
            Field : 0,
            Param : 1,
        ],
        /// | HasDeclSecurity: 2 bits to encode tag | Tag |
        /// | ------------------------------------- | --- |
        /// | `TypeDef`                             | 0   |
        /// | `MethodDef`                           | 1   |
        /// | `Assembly`                            | 2   |
        HasDeclSecurity : 2 = [
            TypeDef : 0,
            MethodDef : 1,
            Assembly : 2,
        ],
        /// | MemberRefParent: 3 bits to encode tag | Tag |
        /// | ------------------------------------- | --- |
        /// | `TypeDef`                             | 0   |
        /// | `TypeRef`                             | 1   |
        /// | `ModuleRef`                           | 2   |
        /// | `MethodDef`                           | 3   |
        /// | `TypeSpec`                            | 4   |
        MemberRefParent : 3 = [
            TypeDef : 0,
            TypeRef : 1,
            ModuleRef : 2,
            MethodDef : 3,
            TypeSpec : 4,
        ],
        /// | HasSemantics: 1 bit to encode tag | Tag |
        /// | --------------------------------- | --- |
        /// | `Event`                           | 0   |
        /// | `Property`                        | 1   |
        HasSemantics : 1 = [
            Event : 0,
            Property : 1,
        ],
        /// | MethodDefOrRef: 1 bit to encode tag | Tag |
        /// | ----------------------------------- | --- |
        /// | `MethodDef`                         | 0   |
        /// | `MemberRef`                         | 1   |
        MethodDefOrRef : 1 = [
            MethodDef : 0,
            MemberRef : 1,
        ],
        /// | MemberForwarded: 1 bit to encode tag | Tag |
        /// | ------------------------------------ | --- |
        /// | `Field`                              | 0   |
        /// | `MethodDef`                          | 1   |
        MemberForwarded : 1 = [
            Field : 0,
            MethodDef : 1,
        ],
        /// | Implementation: 2 bits to encode tag | Tag |
        /// | ------------------------------------ | --- |
        /// | `File`                               | 0   |
        /// | `AssemblyRef`                        | 1   |
        /// | `ExportedType`                       | 2   |
        Implementation : 2 = [
            File : 0,
            AssemblyRef : 1,
            ExportedType : 2,
        ],
        /// | CustomAttributeType: 3 bits to encode tag | Tag |
        /// | ----------------------------------------- | --- |
        /// | `NotUsed`                                 | 0   |
        /// | `NotUsed`                                 | 1   |
        /// | `MethodDef`                               | 2   |
        /// | `MemberRef`                               | 3   |
        /// | `NotUsed`                                 | 4   |
        CustomAttributeType : 3 = [
            MethodDef : 2,
            MemberRef : 3,
        ],
        /// | ResolutionScope: 2 bits to encode tag | Tag |
        /// | ------------------------------------- | --- |
        /// | `Module`                              | 0   |
        /// | `ModuleRef`                           | 1   |
        /// | `AssemblyRef`                         | 2   |
        /// | `TypeRef`                             | 3   |
        ResolutionScope : 2 = [
            Module : 0,
            ModuleRef : 1,
            AssemblyRef : 2,
            TypeRef : 3,
        ],
        /// | TypeOrMethodDef: 1 bit to encode tag | Tag |
        /// | ------------------------------------ | --- |
        /// | `TypeDef`                            | 0   |
        /// | `MethodDef`                          | 1   |
        TypeOrMethodDef : 1 = [
            TypeDef : 0,
            MethodDef : 1,
        ],
    }
}

impl TableEnumDecode for CodedIndexTag {
    type Output = CodedIndex;
    
    fn decode(self, context: &TableDecodeContext, buffer: &mut Buffer) -> Result<CodedIndex, std::io::Error> {
        let index: u32 = if context.get_coded_index_size(self) == 4 {
            buffer.read_u32::<LittleEndian>()?
        } else {
            buffer.read_u16::<LittleEndian>()? as u32
        };

        let data = index >> self.get_tag_size();
        let table = self.get_table_kind((index & 0xff) as u8);
        Ok(CodedIndex::from(table, data))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodedIndex {
    pub table: TableKind,
    pub index: u32,
}

impl CodedIndex {
    pub fn from(table: TableKind, index: u32) -> Self {
        CodedIndex {
            table,
            index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetadataToken {
    UserString(u32),
    Table(TableKind, u32),
}

impl MetadataToken {
    pub fn from_raw(raw: u32) -> Self {
        let table = (raw >> 24) as u8;
        let index = raw & 0x00FFFFFF;

        match table {
            0x70 => MetadataToken::UserString(index),
            _ => MetadataToken::Table(TableKind::from(table), index),
        }
    }

    pub fn to_raw(&self) -> u32 {
        match self {
            MetadataToken::UserString(index) => 0x70 << 24 | index,
            MetadataToken::Table(table, index) => (u8::from(*table) as u32) << 24 | index,
        }
    }

    pub fn read(buffer: &mut Buffer) -> Result<Self, std::io::Error> {
        let raw = buffer.read_u32::<LittleEndian>()?;
        Ok(MetadataToken::from_raw(raw))
    }
}

