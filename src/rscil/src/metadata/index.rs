
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

/// # II.24.2.6 #~ stream 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodedIndexTag {
    /// | TypeDefOrRef: 2 bits to encode tag | Tag |
    /// | ---------------------------------- | --- |
    /// | `TypeDef`                          | 0   |
    /// | `TypeRef`                          | 1   |
    /// | `TypeSpec`                         | 2   |
    TypeDefOrRef,
    /// | HasConstant: 2 bits to encode tag | Tag |
    /// | --------------------------------- | --- |
    /// | `Field`                           | 0   |
    /// | `Param`                           | 1   |
    /// | `Property`                        | 2   |
    HasConstant,
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
    HasCustomAttribute,
    /// | HasFieldMarshall: 1 bit to encode tag | Tag |
    /// | ------------------------------------- | --- |
    /// | `Field`                               | 0   |
    /// | `Param`                               | 1   |
    HasFieldMarshal,
    /// | HasDeclSecurity: 2 bits to encode tag | Tag |
    /// | ------------------------------------- | --- |
    /// | `TypeDef`                             | 0   |
    /// | `MethodDef`                           | 1   |
    /// | `Assembly`                            | 2   |
    HasDeclSecurity,
    /// | MemberRefParent: 3 bits to encode tag | Tag |
    /// | ------------------------------------- | --- |
    /// | `TypeDef`                             | 0   |
    /// | `TypeRef`                             | 1   |
    /// | `ModuleRef`                           | 2   |
    /// | `MethodDef`                           | 3   |
    /// | `TypeSpec`                            | 4   |
    MemberRefParent,
    /// | HasSemantics: 1 bit to encode tag | Tag |
    /// | --------------------------------- | --- |
    /// | `Event`                           | 0   |
    /// | `Property`                        | 1   |
    HasSemantics,
    /// | MethodDefOrRef: 1 bit to encode tag | Tag |
    /// | ----------------------------------- | --- |
    /// | `MethodDef`                         | 0   |
    /// | `MemberRef`                         | 1   |
    MethodDefOrRef,
    /// | MemberForwarded: 1 bit to encode tag | Tag |
    /// | ------------------------------------ | --- |
    /// | `Field`                              | 0   |
    /// | `MethodDef`                          | 1   |
    MemberForwarded,
    /// | Implementation: 2 bits to encode tag | Tag |
    /// | ------------------------------------ | --- |
    /// | `File`                               | 0   |
    /// | `AssemblyRef`                        | 1   |
    /// | `ExportedType`                       | 2   |
    Implementation,
    /// | CustomAttributeType: 3 bits to encode tag | Tag |
    /// | ----------------------------------------- | --- |
    /// | `NotUsed`                                 | 0   |
    /// | `NotUsed`                                 | 1   |
    /// | `MethodDef`                               | 2   |
    /// | `MemberRef`                               | 3   |
    /// | `NotUsed`                                 | 4   |
    CustomAttributeType,
    /// | ResolutionScope: 2 bits to encode tag | Tag |
    /// | ------------------------------------- | --- |
    /// | `Module`                              | 0   |
    /// | `ModuleRef`                           | 1   |
    /// | `AssemblyRef`                         | 2   |
    /// | `TypeRef`                             | 3   |
    ResolutionScope,
    /// | TypeOrMethodDef: 1 bit to encode tag | Tag |
    /// | ------------------------------------ | --- |
    /// | `TypeDef`                            | 0   |
    /// | `MethodDef`                          | 1   |
    TypeOrMethodDef,
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

impl CodedIndexTag {
    pub fn get_tag_size(&self) -> u8 {
        match self {
            CodedIndexTag::TypeDefOrRef => 2,
            CodedIndexTag::HasConstant => 2,
            CodedIndexTag::HasCustomAttribute => 5,
            CodedIndexTag::HasFieldMarshal => 1,
            CodedIndexTag::HasDeclSecurity => 2,
            CodedIndexTag::MemberRefParent => 3,
            CodedIndexTag::HasSemantics => 1,
            CodedIndexTag::MethodDefOrRef => 1,
            CodedIndexTag::MemberForwarded => 1,
            CodedIndexTag::Implementation => 2,
            CodedIndexTag::CustomAttributeType => 3,
            CodedIndexTag::ResolutionScope => 2,
            CodedIndexTag::TypeOrMethodDef => 1,
        }
    }

    pub fn get_table_kind(&self, data: u8) -> TableKind {
        // Clean data with tag size
        let data = data & (0xFF >> (8 - self.get_tag_size()));

        match self {
            CodedIndexTag::TypeDefOrRef => {
                match data {
                    0 => TableKind::TypeDef,
                    1 => TableKind::TypeRef,
                    2 => TableKind::TypeSpec,
                    _ => panic!("Invalid TypeDefOrRef tag: {}", data),
                }
            },
            CodedIndexTag::HasConstant => {
                match data {
                    0 => TableKind::Field,
                    1 => TableKind::Param,
                    2 => TableKind::Property,
                    _ => panic!("Invalid HasConstant tag: {}", data),
                }
            },
            CodedIndexTag::HasCustomAttribute => {
                match data {
                    0 => TableKind::MethodDef,
                    1 => TableKind::Field,
                    2 => TableKind::TypeRef,
                    3 => TableKind::TypeDef,
                    4 => TableKind::Param,
                    5 => TableKind::InterfaceImpl,
                    6 => TableKind::MemberRef,
                    7 => TableKind::Module,
                    // 8 => TableKind::Permission, // TODO: What is this?
                    9 => TableKind::Property,
                    10 => TableKind::Event,
                    11 => TableKind::StandAloneSig,
                    12 => TableKind::ModuleRef,
                    13 => TableKind::TypeSpec,
                    14 => TableKind::Assembly,
                    15 => TableKind::AssemblyRef,
                    16 => TableKind::File,
                    17 => TableKind::ExportedType,
                    18 => TableKind::ManifestResource,
                    19 => TableKind::GenericParam,
                    20 => TableKind::GenericParamConstraint,
                    21 => TableKind::MethodSpec,
                    _ => panic!("Invalid HasCustomAttribute tag: {}", data),
                }
            },
            CodedIndexTag::HasFieldMarshal => {
                match data {
                    0 => TableKind::Field,
                    1 => TableKind::Param,
                    _ => panic!("Invalid HasFieldMarshal tag: {}", data),
                }
            },
            CodedIndexTag::HasDeclSecurity => {
                match data {
                    0 => TableKind::TypeDef,
                    1 => TableKind::MethodDef,
                    2 => TableKind::Assembly,
                    _ => panic!("Invalid HasDeclSecurity tag: {}", data),
                }
            },
            CodedIndexTag::MemberRefParent => {
                match data {
                    0 => TableKind::TypeDef,
                    1 => TableKind::TypeRef,
                    2 => TableKind::ModuleRef,
                    3 => TableKind::MethodDef,
                    4 => TableKind::TypeSpec,
                    _ => panic!("Invalid MemberRefParent tag: {}", data),
                }
            },
            CodedIndexTag::HasSemantics => {
                match data {
                    0 => TableKind::Event,
                    1 => TableKind::Property,
                    _ => panic!("Invalid HasSemantics tag: {}", data),
                }
            },
            CodedIndexTag::MethodDefOrRef => {
                match data {
                    0 => TableKind::MethodDef,
                    1 => TableKind::MemberRef,
                    _ => panic!("Invalid MethodDefOrRef tag: {}", data),
                }
            },
            CodedIndexTag::MemberForwarded => {
                match data {
                    0 => TableKind::Field,
                    1 => TableKind::MethodDef,
                    _ => panic!("Invalid MemberForwarded tag: {}", data),
                }
            },
            CodedIndexTag::Implementation => {
                match data {
                    0 => TableKind::File,
                    1 => TableKind::AssemblyRef,
                    2 => TableKind::ExportedType,
                    _ => panic!("Invalid Implementation tag: {}", data),
                }
            },
            CodedIndexTag::CustomAttributeType => {
                match data {
                    2 => TableKind::MethodDef,
                    3 => TableKind::MemberRef,
                    _ => panic!("Invalid CustomAttributeType tag: {}", data),
                }
            },
            CodedIndexTag::ResolutionScope => {
                match data {
                    0 => TableKind::Module,
                    1 => TableKind::ModuleRef,
                    2 => TableKind::AssemblyRef,
                    3 => TableKind::TypeRef,
                    _ => panic!("Invalid ResolutionScope tag: {}", data),
                }
            },
            CodedIndexTag::TypeOrMethodDef => {
                match data {
                    0 => TableKind::TypeDef,
                    1 => TableKind::MethodDef,
                    _ => panic!("Invalid TypeOrMethodDef tag: {}", data),
                }
            },
        }
    }

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If *e* is a *coded index* that points into table *t<sub>i</sub>* out of *n* possible tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, then it 
    ///   is stored as e << (log n) | tag{*t<sub>0</sub>*, ...*t<sub>n-1</sub>*}\[*t<sub>i</sub>*] using 2 bytes if the maximum number 
    ///   of rows of tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, is less than 2<sup>(16 – (log n))</sup>, and using 4 bytes otherwise.
    pub fn is_big_index(&self, row_count: impl Fn(TableKind) -> u32) -> bool {
        match self {
            CodedIndexTag::TypeDefOrRef => {
                let max = row_count(TableKind::TypeDef)
                    .max(row_count(TableKind::TypeRef)
                    .max(row_count(TableKind::TypeSpec)));
                max > 2u32.pow(16 - 2)
            },
            CodedIndexTag::HasConstant => {
                let max = row_count(TableKind::Field)
                    .max(row_count(TableKind::Param)
                    .max(row_count(TableKind::Property)));
                max > 2u32.pow(16 - 2)
            },
            CodedIndexTag::HasCustomAttribute => {
                let max = row_count(TableKind::MethodDef)
                    .max(row_count(TableKind::Field)
                    .max(row_count(TableKind::TypeRef)
                    .max(row_count(TableKind::TypeDef)
                    .max(row_count(TableKind::Param)
                    .max(row_count(TableKind::InterfaceImpl)
                    .max(row_count(TableKind::MemberRef)
                    .max(row_count(TableKind::Module)
                    .max(row_count(TableKind::Property)
                    .max(row_count(TableKind::Event)
                    .max(row_count(TableKind::StandAloneSig)
                    .max(row_count(TableKind::ModuleRef)
                    .max(row_count(TableKind::TypeSpec)
                    .max(row_count(TableKind::Assembly)
                    .max(row_count(TableKind::AssemblyRef)
                    .max(row_count(TableKind::File)
                    .max(row_count(TableKind::ExportedType)
                    .max(row_count(TableKind::ManifestResource)
                    .max(row_count(TableKind::GenericParam)
                    .max(row_count(TableKind::GenericParamConstraint)
                    .max(row_count(TableKind::MethodSpec)))))))))))))))))))));

                max > 2u32.pow(16 - 5)
            },
            CodedIndexTag::HasFieldMarshal => {
                let max = row_count(TableKind::Field)
                    .max(row_count(TableKind::Param));
                max > 2u32.pow(16 - 1)
            },
            CodedIndexTag::HasDeclSecurity => {
                let max = row_count(TableKind::TypeDef)
                    .max(row_count(TableKind::MethodDef)
                    .max(row_count(TableKind::Assembly)));
                max > 2u32.pow(16 - 2)
            },
            CodedIndexTag::MemberRefParent => {
                let max = row_count(TableKind::TypeDef)
                    .max(row_count(TableKind::TypeRef)
                    .max(row_count(TableKind::ModuleRef)
                    .max(row_count(TableKind::MethodDef)
                    .max(row_count(TableKind::TypeSpec)))));

                max > 2u32.pow(16 - 3)
            },
            CodedIndexTag::HasSemantics => {
                let max = row_count(TableKind::Event)
                    .max(row_count(TableKind::Property));

                max > 2u32.pow(16 - 1)
            },
            CodedIndexTag::MethodDefOrRef => {
                let max = row_count(TableKind::MethodDef)
                    .max(row_count(TableKind::MemberRef));

                max > 2u32.pow(16 - 1)
            },
            CodedIndexTag::MemberForwarded => {
                let max = row_count(TableKind::Field)
                    .max(row_count(TableKind::MethodDef));

                max > 2u32.pow(16 - 1)
            },
            CodedIndexTag::Implementation => {
                let max = row_count(TableKind::File)
                    .max(row_count(TableKind::AssemblyRef)
                    .max(row_count(TableKind::ExportedType)));

                max > 2u32.pow(16 - 2)
            },
            CodedIndexTag::CustomAttributeType => {
                let max = row_count(TableKind::MethodDef)
                    .max(row_count(TableKind::MemberRef));

                max > 2u32.pow(16 - 3)
            },
            CodedIndexTag::ResolutionScope => {
                let max = row_count(TableKind::Module)
                    .max(row_count(TableKind::ModuleRef)
                    .max(row_count(TableKind::AssemblyRef)
                    .max(row_count(TableKind::TypeRef))));

                max > 2u32.pow(16 - 2)
            },
            CodedIndexTag::TypeOrMethodDef => {
                let max = row_count(TableKind::TypeDef)
                    .max(row_count(TableKind::MethodDef));

                max > 2u32.pow(16 - 1)
            },
        }
    }

    pub fn iter() -> Iter<'static, (CodedIndexTag, u8)> {
        static ITER: [(CodedIndexTag, u8); 13] = [
            (CodedIndexTag::TypeDefOrRef, 2),
            (CodedIndexTag::HasConstant, 2),
            (CodedIndexTag::HasCustomAttribute, 5),
            (CodedIndexTag::HasFieldMarshal, 1),
            (CodedIndexTag::HasDeclSecurity, 2),
            (CodedIndexTag::MemberRefParent, 3),
            (CodedIndexTag::HasSemantics, 1),
            (CodedIndexTag::MethodDefOrRef, 1),
            (CodedIndexTag::MemberForwarded, 1),
            (CodedIndexTag::Implementation, 2),
            (CodedIndexTag::CustomAttributeType, 3),
            (CodedIndexTag::ResolutionScope, 2),
            (CodedIndexTag::TypeOrMethodDef, 1),
        ];
        ITER.iter()
    }

    /// # II.24.2.6 #~ stream 
    /// 
    /// [...]
    /// 
    /// * If *e* is a *coded index* that points into table *t<sub>i</sub>* out of *n* possible tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, then it 
    ///   is stored as e << (log n) | tag{*t<sub>0</sub>*, ...*t<sub>n-1</sub>*}[*t<sub>i</sub>] using 2 bytes if the maximum number 
    ///   of rows of tables *t<sub>0</sub>*, ...*t<sub>n-1</sub>*, is less than 2<sup>(16 – (log n))</sup>, and using 4 bytes otherwise.
    ///   The family of finite maps tag{*t<sub>0</sub>, ...*t<sub>n-1</sub>*} is defined below. Note that decoding a physical 
    ///   row requires the inverse of this mapping. [For example, the Parent column of the 
    ///   *Constant* table indexes a row in the *Field*, *Param*, or *Property* tables.  The actual 
    ///   table is encoded into the low 2 bits of the number, using the values: 0 => *Field*, 1 => 
    ///   *Param*, 2 => *Property*. The remaining bits hold the actual row number being 
    ///   indexed. For example, a value of `0x321`, indexes row number `0xC8` in the *Param* table.]
    pub fn read(self, buffer: &mut Buffer, context: &TableDecodeContext) -> Result<CodedIndex, std::io::Error> {
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

