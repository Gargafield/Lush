use std::{fs::File, io::BufReader};

use metadata::bufreader_extension::BufReaderExtension;

use crate::*;

/// II.22.2 Assembly : 0x20
/// [...]
///  
/// 1. The Assembly table shall contain zero or one row [ERROR]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyTable { pub row: Option<AssemblyRow> }

#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyRefTable(Vec<AssemblyRefRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantTable(Vec<ConstantRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct CustomAttributeTable(Vec<CustomAttributeRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct FieldTable(Vec<FieldRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct MemberRefTable(Vec<MemberRefRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefTable(Vec<MethodDefRow>);

/// II.22.30 Module : 0x00
/// [...]
/// 
/// 1. The Module table shall contain one and only one row [ERROR] 
pub struct ModuleTable(pub ModuleRow);

#[derive(Debug, Clone, PartialEq)]
pub struct ParamTable(Vec<ParamRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefTable(Vec<TypeDefRow>);

#[derive(Debug, Clone, PartialEq)]
pub struct TypeRefTable(Vec<TypeRefRow>);

fn read_rows<T: TableRow>(buffer: &mut BufReader<File>, row_count: u32) -> Result<Vec<T>, std::io::Error> {
    let mut rows = Vec::new();
    for _ in 0..row_count {
        rows.push(T::read_from(buffer)?);
    }
    Ok(rows)
}

macro_rules! table_impl {
    ($class:ident, $read:expr) => {
        impl $class {
            pub fn read_from(buffer: &mut BufReader<File>, row_count: u32) -> Result<Self, std::io::Error> {
                $read(buffer, row_count)
            }
        }
    };
}


table_impl!(AssemblyTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(AssemblyTable {
        row: if row_count <= 0 {None} else { Some(AssemblyRow::read_from(buffer)?) }
    })
});

table_impl!(AssemblyRefTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(AssemblyRefTable(read_rows(buffer, row_count)?))
});

table_impl!(ConstantTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(ConstantTable(read_rows(buffer, row_count)?))
});

table_impl!(CustomAttributeTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(CustomAttributeTable(read_rows(buffer, row_count)?))
});

table_impl!(FieldTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(FieldTable(read_rows(buffer, row_count)?))
});

table_impl!(MemberRefTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(MemberRefTable(read_rows(buffer, row_count)?))
});

table_impl!(MethodDefTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(MethodDefTable(read_rows(buffer, row_count)?))
});

table_impl!(ModuleTable, |buffer: &mut BufReader<File>, _row_count: u32| {
    Ok(ModuleTable(ModuleRow::read_from(buffer)?))
});

table_impl!(ParamTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(ParamTable(read_rows(buffer, row_count)?))
});

table_impl!(TypeDefTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(TypeDefTable(read_rows(buffer, row_count)?))
});

table_impl!(TypeRefTable, |buffer: &mut BufReader<File>, row_count: u32| {
    Ok(TypeRefTable(read_rows(buffer, row_count)?))
});

pub trait TableRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<Self, std::io::Error> where Self: Sized;
}

/// # II.22.2 Assembly : 0x20
/// The *Assembly* table has the following columns: 
/// * *HashAlgId* (a 4-byte constant of type AssemblyHashAlgorithm, §II.23.1.1)
/// * *MajorVersion*, *MinorVersion*, *BuildNumber*, *RevisionNumber* (each being 2-byte constants) 
/// * *Flags* (a 4-byte bitmask of type AssemblyFlags, §II.23.1.2)
/// * *PublicKey* (an index into the Blob heap)
/// * *Name* (an index into the String heap)
/// * *Culture* (an index into the String heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyRow {
    pub hash_alg_id: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub flags: AssemblyFlags,
    pub public_key: BlobIndex,
    pub name: StringIndex,
    pub culture: StringIndex,
}

impl TableRow for AssemblyRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<AssemblyRow, std::io::Error> {
        Ok(AssemblyRow {
            hash_alg_id: buffer.read_u32()?,
            major_version: buffer.read_u16()?,
            minor_version: buffer.read_u16()?,
            build_number: buffer.read_u16()?,
            revision_number: buffer.read_u16()?,
            flags: AssemblyFlags::from(buffer.read_u32()? as u16),
            public_key: BlobIndex::read(buffer)?,
            name: StringIndex::read(buffer)?,
            culture: StringIndex::read(buffer)?,
        })
    }
}

// TODO: AssemblyOS
// TODO: AssemblyProcessor

/// # II.22.8 AssemblyRef : 0x23
/// The *AssemblyRef* table has the following columns: 
/// * *MajorVersion*, *MinorVersion*, *BuildNumber*, *RevisionNumber* (each being 2-byte constants)
/// * *Flags* (a 4-byte bitmask of type AssemblyFlags, §II.23.1.2)
/// * *PublicKeyOrToken* (an index into the Blob heap, indicating the public key or token that identifies the author of this Assembly)
/// * *Name* (an index into the String heap)
/// * *Culture* (an index into the String heap)
/// * *HashValue* (an index into the Blob heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyRefRow {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub flags: AssemblyFlags,
    pub public_key_or_token: BlobIndex,
    pub name: StringIndex,
    pub culture: StringIndex,
    pub hash_value: BlobIndex,
}

impl TableRow for AssemblyRefRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<AssemblyRefRow, std::io::Error> {
        Ok(AssemblyRefRow {
            major_version: buffer.read_u16()?,
            minor_version: buffer.read_u16()?,
            build_number: buffer.read_u16()?,
            revision_number: buffer.read_u16()?,
            flags: AssemblyFlags::from(buffer.read_u32()? as u16),
            public_key_or_token: BlobIndex::read(buffer)?,
            name: StringIndex::read(buffer)?,
            culture: StringIndex::read(buffer)?,
            hash_value: BlobIndex::read(buffer)?,
        })
    }
}

// TODO: AssemblyRefOS
// TODO: AssemblyRefProcessor
// TODO: ClassLayout

/// # II.22.9 Constant : 0x0B
/// 
/// The *Constant* table is used to store compile-time, constant values for fields, parameters, and properties. 
/// 
/// The Constant table has the following columns: 
/// * Type (a 1-byte constant, followed by a 1-byte padding zero); see §II.23.1.16. 
///   The encoding of *Type* for the **nullref** value for FieldInit in ilasm (§II.16.2) is 
///   `ELEMENT_TYPE_CLASS` with a Value of a 4-byte zero. Unlike uses of 
///   `ELEMENT_TYPE_CLASS` in signatures, this one is not followed by a type token.
/// * *Parent* (an index into the *Param*, *Field*, or *Property* table; more precisely, a *HasConstant* (§II.24.2.6) coded index) 
/// * *Value* (an index into the Blob heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantRow {
    pub type_: u16,
    pub parent: CodedIndex,
    pub value: BlobIndex,
}

impl TableRow for ConstantRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<ConstantRow, std::io::Error> {
        Ok(ConstantRow {
            type_: buffer.read_u16()?,
            parent: CodedIndex::read(buffer, CodedIndexTag::HasConstant)?,
            value: BlobIndex::read(buffer)?,
        })
    }
}

/// # II.22.10 CustomAttribute : 0x0C
/// 
/// The *CustomAttribute* table has the following columns:
/// 
/// * *Parent* (an index into a metadata table that has an associated *HasCustomAttribute* (§II.24.2.6) coded index)
/// * *Type* (an index into the *MethodDef* or *MemberRef* table; more precisely, a *CustomAttributeType* (§II.24.2.6) coded index). 
/// * *Value* (an index into the Blob heap). 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomAttributeRow {
    pub parent: CodedIndex,
    pub type_: CodedIndex,
    pub value: BlobIndex,
}

impl TableRow for CustomAttributeRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<CustomAttributeRow, std::io::Error> {
        Ok(CustomAttributeRow {
            parent: CodedIndex::read(buffer, CodedIndexTag::HasCustomAttribute)?,
            type_: CodedIndex::read(buffer, CodedIndexTag::CustomAttributeType)?,
            value: BlobIndex::read(buffer)?,
        })
    }
}

// TODO: DeclSecurity
// TODO: EventMap
// TODO: Event
// TODO: ExportedType

/// # II.22.15 Field : 0x04
/// 
/// The *Field* table has the following columns:
/// 
/// * *Flags* (a 2-byte bitmask of type *FieldAttributes*, §II.23.1.5)
/// * *Name* (an index into the String heap)
/// * *Signature* (an index into the Blob heap)
/// 
/// Conceptually, each row in the Field table is owned by one, and only one, row in the TypeDef table. 
/// However, the owner of any row in the Field table is not stored anywhere in the Field table itself. 
/// There is merely a ‘forward-pointer’ from each row in the TypeDef table (the FieldList column), as 
/// shown in the following illustration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldRow {
    pub flags: FieldAttributes,
    pub name: StringIndex,
    pub signature: BlobIndex,
}

impl TableRow for FieldRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<FieldRow, std::io::Error> {
        Ok(FieldRow {
            flags: FieldAttributes::from(buffer.read_u16()?),
            name: StringIndex::read(buffer)?,
            signature: BlobIndex::read(buffer)?,
        })
    }
}

// TODO: FieldLayout
// TODO: FieldMarshal 
// TODO: FieldRVA
// TODO: File
// TODO: GenericParam
// TODO: GenericParamConstraint
// TODO: ImplMap
// TODO: InterfaceImpl
// TODO: ManifestResource

/// # II.22.25 MemberRef : 0x0A
///  
/// The *MemberRef* table combines two sorts of references, to Methods and to Fields of a class, known as 
/// 'MethodRef' and 'FieldRef', respectively. The *MemberRef* table has the following columns: 
/// * *Class* (an index into the *MethodDef*, *ModuleRef*, *TypeDef*, *TypeRef*, or *TypeSpec* 
///   tables; more precisely, a MemberRefParent  (§II.24.2.6) coded index) 
/// * *Name* (an index into the String heap)
/// * *Signature* (an index into the Blob heap)
/// 
/// An entry is made into the MemberRef table whenever a reference is made in the CIL code to a 
/// method or field which is defined in another module or assembly.  (Also, an entry is made for a 
/// call to a method with a VARARG signature, even when it is defined in the same module as the call site.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemberRefRow {
    pub class: CodedIndex,
    pub name: StringIndex,
    pub signature: BlobIndex,
}

impl TableRow for MemberRefRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<MemberRefRow, std::io::Error> {
        Ok(MemberRefRow {
            class: CodedIndex::read(buffer, CodedIndexTag::MemberRefParent)?,
            name: StringIndex::read(buffer)?,
            signature: BlobIndex::read(buffer)?,
        })
    }
}

/// # II.22.26 MethodDef : 0x06
/// 
/// The *MethodDef* table has the following columns: 
/// * *RVA* (a 4-byte constant)
/// * *ImplFlags* (a 2-byte bitmask of type *MethodImplAttributes*, §II.23.1.10)
/// * *Flags* (a 2-byte bitmask of type *MethodAttributes*, §II.23.1.10)
/// * *Name* (an index into the String heap)
/// * *Signature* (an index into the Blob heap)
/// * *ParamList* (an index into the *Param* table). It marks the beginning of a contiguous run of
///   Parameters owned by this method. The run continues to the smaller of:
///     * the last row of the Param table 
///     * the next run of Parameters, found by inspecting the *ParamList* of the next row in the *MethodDef* table
///
/// Conceptually, every row in the *MethodDef* table is owned by one, and only one, row in the *TypeDef* table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MethodDefRow {
    pub rva: u32,
    pub impl_flags: MethodImplAttributes,
    pub flags: MethodAttributes,
    pub name: StringIndex,
    pub signature: BlobIndex,
    pub param_list: CodedIndex,
}

impl TableRow for MethodDefRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<MethodDefRow, std::io::Error> {
        Ok(MethodDefRow {
            rva: buffer.read_u32()?,
            impl_flags: MethodImplAttributes::from(buffer.read_u16()?),
            flags: MethodAttributes::from(buffer.read_u16()?),
            name: StringIndex::read(buffer)?,
            signature: BlobIndex::read(buffer)?,
            param_list: CodedIndex {
                index: buffer.read_u16()? as u32,
                table: TableKind::Param,
            },
        })
    }
}

// TODO: MethodImpl
// TODO: MethodSemantics
// TODO: MethodSpec

/// # II.22.30 Module : 0x00
/// 
/// The *Module* table has the following columns:
/// * *Generation* (a 2-byte value, reserved, shall be zero) 
/// * *Name* (an index into the String heap)
/// * *Mvid*  (an index into the Guid heap; simply a Guid used to distinguish between two versions of the same module)
/// * *EncId* (an index into the Guid heap; reserved, shall be zero) 
/// * *EncBaseId* (an index into the Guid heap; reserved, shall be zero)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleRow {
    pub generation: u16,
    pub name: StringIndex,
    pub mvid: GuidIndex,
    pub enc_id: GuidIndex,
    pub enc_base_id: GuidIndex,
}

impl TableRow for ModuleRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<ModuleRow, std::io::Error> {
        Ok(ModuleRow {
            generation: buffer.read_u16()?,
            name: StringIndex::read(buffer)?,
            mvid: GuidIndex::read(buffer)?,
            enc_id: GuidIndex::read(buffer)?,
            enc_base_id: GuidIndex::read(buffer)?,
        })
    }
}

// TODO: ModuleRef
// TODO: NestedClass

/// II.22.33 Param : 0x08
/// 
/// The *Param* table has the following columns: 
/// * *Flags* (a 2-byte bitmask of type ParamAttributes, §II.23.1.13)
/// * *Sequence* (a 2-byte constant)
/// * *Name* (an index into the String heap)
/// 
/// Conceptually, every row in the Param table is owned by one, and only one, row in the *MethodDef* table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParamRow {
    pub flags: ParamAttributes,
    pub sequence: u16,
    pub name: StringIndex,
}

impl TableRow for ParamRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<ParamRow, std::io::Error> {
        Ok(ParamRow {
            flags: ParamAttributes::from(buffer.read_u16()?),
            sequence: buffer.read_u16()?,
            name: StringIndex::read(buffer)?,
        })
    }
}

// TODO: Property
// TODO: PropertyMap
// TODO: StandAloneSig

/// # II.22.37 TypeDef : 0x02
/// The *TypeDef* table has the following columns:
/// * *Flags* (a 4-byte bitmask of type TypeAttributes, §II.23.1.15)
/// * *TypeName* (an index into the String heap)
/// * *TypeNamespace* (an index into the String heap)
/// * *Extends* (an index into the *TypeDef*, *TypeRef*, or *TypeSpec* table; more precisely, a *TypeDefOrRef* (§II.24.2.6) coded index)
/// * *FieldList* (an index into the *Field* table; it marks the first of a contiguous run of Fields owned by this Type).
///   The run continues to the smaller of:
///    * the last row of the *Field* table 
///    * the next run of Fields, found by inspecting the *FieldList* of the next row in the *TypeDef* table
/// * *MethodList* (an index into the *MethodDef* table; it marks the first of a contiguous run of Methods owned by this Type).
///   The run continues to the smaller of:
///   * the last row of the *MethodDef* table
///   * the next run of Methods, found by inspecting the *MethodList* of the next row in the *TypeDef* table
/// 
/// The first row of the TypeDef table represents the pseudo class that acts as parent for functions 
/// and variables defined at module scope.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeDefRow {
    pub flags: TypeAttributes,
    pub type_name: StringIndex,
    pub type_namespace: StringIndex,
    pub extends: CodedIndex,
    pub field_list: CodedIndex,
    pub method_list: CodedIndex,
}

impl TableRow for TypeDefRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<TypeDefRow, std::io::Error> {
        Ok(TypeDefRow {
            flags: TypeAttributes::from(buffer.read_u32()?),
            type_name: StringIndex::read(buffer)?,
            type_namespace: StringIndex::read(buffer)?,
            extends: CodedIndex::read(buffer, CodedIndexTag::TypeDefOrRef)?,
            field_list: CodedIndex {
                index: buffer.read_u16()? as u32,
                table: TableKind::Field,
            },
            method_list: CodedIndex {
                index: buffer.read_u16()? as u32,
                table: TableKind::MethodDef,
            },
        })
    }
}

/// # II.22.38 TypeRef : 0x01
/// 
/// The *TypeRef* table has the following columns:
/// * *ResolutionScope* (an index into a *Module*, *ModuleRef*, *AssemblyRef* or *TypeRef* table, 
///   or null; more precisely, a *ResolutionScope* (§II.24.2.6) coded index)
/// * *TypeName* (an index into the String heap)
/// * *TypeNamespace* (an index into the String heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeRefRow {
    pub resolution_scope: CodedIndex,
    pub type_name: StringIndex,
    pub type_namespace: StringIndex,
}

impl TableRow for TypeRefRow {
    fn read_from(buffer: &mut BufReader<File>) -> Result<TypeRefRow, std::io::Error> {
        Ok(TypeRefRow {
            resolution_scope: CodedIndex::read(buffer, CodedIndexTag::ResolutionScope)?,
            type_name: StringIndex::read(buffer)?,
            type_namespace: StringIndex::read(buffer)?,
        })
    }
}

// TODO: TypeSpec
