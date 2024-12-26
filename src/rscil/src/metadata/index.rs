use std::{fs::File, io::BufReader};

use super::{bufreader_extension::BufReaderExtension, TableKind};

// TODO: Dynamic size for indexes, see II.24.2.6 #~ stream

pub trait Index {
    fn read(buffer: &mut BufReader<File>) -> Result<Self, std::io::Error> where Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringIndex(pub u32);

impl Index for StringIndex {
    fn read(buffer: &mut BufReader<File>) -> Result<StringIndex, std::io::Error> {
        Ok(StringIndex(buffer.read_u16()? as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GuidIndex(pub u32);
impl Index for GuidIndex {
    fn read(buffer: &mut BufReader<File>) -> Result<GuidIndex, std::io::Error> {
        Ok(GuidIndex(buffer.read_u16()? as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlobIndex(pub u32);
impl Index for BlobIndex {
    fn read(buffer: &mut BufReader<File>) -> Result<BlobIndex, std::io::Error> {
        Ok(BlobIndex(buffer.read_u16()? as u32))
    }
}

/// # II.24.2.6 #~ stream 
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodedIndex {
    pub table: TableKind,
    pub index: u32,
}

impl CodedIndex {
    pub fn read(buffer: &mut BufReader<File>, tag: CodedIndexTag) -> Result<CodedIndex, std::io::Error> {
        let index = buffer.read_u16()?;
        
        let data_index = index >> tag.get_tag_size();
        let table = tag.get_table_kind((index & 0xFF) as u8);

        Ok(CodedIndex {
            table,
            index: data_index as u32,
        })
    }
}
