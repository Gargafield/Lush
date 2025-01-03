
use crate::*;

#[macro_export]
macro_rules! cast_table {
    ($table:ident, $expr:expr) => {
        match $expr {
            Table::$table(x) => x,
            _ => panic!("Failed to cast table to {}", stringify!($table))
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Table {
    /// # II.22.2 Assembly : 0x20
    /// [...]
    ///  
    /// 1. The Assembly table shall contain zero or one row [ERROR]
    Assembly(Option<AssemblyRow>),
    AssemblyRef(Vec<AssemblyRefRow>),
    ClassLayout(Vec<ClassLayoutRow>),
    Constant(Vec<ConstantRow>),
    CustomAttribute(Vec<CustomAttributeRow>),
    DeclSecurity(Vec<DeclSecurityRow>),
    EventMap(Vec<EventMapRow>),
    Event(Vec<EventRow>),
    ExportedType(Vec<ExportedTypeRow>),
    Field(Vec<FieldRow>),
    FieldLayout(Vec<FieldLayoutRow>),
    FieldMarshal(Vec<FieldMarshalRow>),
    FieldRVA(Vec<FieldRVARow>),
    File(Vec<FileRow>),
    GenericParam(Vec<GenericParamRow>),
    GenericParamConstraint(Vec<GenericParamConstraintRow>),
    ImplMap(Vec<ImplMapRow>),
    InterfaceImpl(Vec<InterfaceImplRow>),
    ManifestResource(Vec<ManifestResourceRow>),
    MemberRef(Vec<MemberRefRow>),
    MethodDef(Vec<MethodDefRow>),
    MethodImpl(Vec<MethodImplRow>),
    MethodSemantics(Vec<MethodSemanticsRow>),
    MethodSpec(Vec<MethodSpecRow>),
    /// II.22.30 Module : 0x00
    /// [...]
    /// 
    /// 1. The Module table shall contain one and only one row [ERROR] 
    Module(ModuleRow),
    ModuleRef(Vec<ModuleRefRow>),
    NestedClass(Vec<NestedClassRow>),
    Param(Vec<ParamRow>),
    Property(Vec<PropertyRow>),
    PropertyMap(Vec<PropertyMapRow>),
    StandAloneSig(Vec<StandAloneSigRow>),
    TypeDef(Vec<TypeDefRow>),
    TypeRef(Vec<TypeRefRow>),
    TypeSpec(Vec<TypeSpecRow>),
}

impl Table {
    pub fn read(buffer: &mut PeParser, kind: TableKind, row_count: u32) -> Result<Table, std::io::Error> {
        match kind {
            TableKind::Assembly => Ok(Table::Assembly(read_single_row(buffer, row_count)?)),
            TableKind::AssemblyOS => unimplemented!(),
            TableKind::AssemblyProcessor => unimplemented!(),
            TableKind::AssemblyRef => Ok(Table::AssemblyRef(read_rows(buffer, row_count)?)),
            TableKind::AssemblyRefOS => unimplemented!(),
            TableKind::AssemblyRefProcessor => unimplemented!(),
            TableKind::ClassLayout => Ok(Table::ClassLayout(read_rows(buffer, row_count)?)),
            TableKind::Constant => Ok(Table::Constant(read_rows(buffer, row_count)?)),
            TableKind::CustomAttribute => Ok(Table::CustomAttribute(read_rows(buffer, row_count)?)),
            TableKind::DeclSecurity => Ok(Table::DeclSecurity(read_rows(buffer, row_count)?)),
            TableKind::EventMap => Ok(Table::EventMap(read_rows(buffer, row_count)?)),
            TableKind::Event => Ok(Table::Event(read_rows(buffer, row_count)?)),
            TableKind::ExportedType => Ok(Table::ExportedType(read_rows(buffer, row_count)?)),
            TableKind::Field => Ok(Table::Field(read_rows(buffer, row_count)?)),
            TableKind::FieldLayout => Ok(Table::FieldLayout(read_rows(buffer, row_count)?)),
            TableKind::FieldMarshal => Ok(Table::FieldMarshal(read_rows(buffer, row_count)?)),
            TableKind::FieldRVA => Ok(Table::FieldRVA(read_rows(buffer, row_count)?)),
            TableKind::File => Ok(Table::File(read_rows(buffer, row_count)?)),
            TableKind::GenericParam => Ok(Table::GenericParam(read_rows(buffer, row_count)?)),
            TableKind::GenericParamConstraint => Ok(Table::GenericParamConstraint(read_rows(buffer, row_count)?)),
            TableKind::ImplMap => Ok(Table::ImplMap(read_rows(buffer, row_count)?)),
            TableKind::InterfaceImpl => Ok(Table::InterfaceImpl(read_rows(buffer, row_count)?)),
            TableKind::ManifestResource => Ok(Table::ManifestResource(read_rows(buffer, row_count)?)),
            TableKind::MemberRef => Ok(Table::MemberRef(read_rows(buffer, row_count)?)),
            TableKind::MethodDef => Ok(Table::MethodDef(read_rows(buffer, row_count)?)),
            TableKind::MethodImpl => Ok(Table::MethodImpl(read_rows(buffer, row_count)?)),
            TableKind::MethodSemantics => Ok(Table::MethodSemantics(read_rows(buffer, row_count)?)),
            TableKind::MethodSpec => Ok(Table::MethodSpec(read_rows(buffer, row_count)?)),
            TableKind::Module => Ok(Table::Module(ModuleRow::read_from(buffer)?)),
            TableKind::ModuleRef => Ok(Table::ModuleRef(read_rows(buffer, row_count)?)),
            TableKind::NestedClass => Ok(Table::NestedClass(read_rows(buffer, row_count)?)),
            TableKind::Param => Ok(Table::Param(read_rows(buffer, row_count)?)),
            TableKind::Property => Ok(Table::Property(read_rows(buffer, row_count)?)),
            TableKind::PropertyMap => Ok(Table::PropertyMap(read_rows(buffer, row_count)?)),
            TableKind::StandAloneSig => Ok(Table::StandAloneSig(read_rows(buffer, row_count)?)),
            TableKind::TypeDef => Ok(Table::TypeDef(read_rows(buffer, row_count)?)),
            TableKind::TypeRef => Ok(Table::TypeRef(read_rows(buffer, row_count)?)),
            TableKind::TypeSpec => Ok(Table::TypeSpec(read_rows(buffer, row_count)?)),
        }
    }
}

fn read_rows<T: TableRow>(buffer: &mut PeParser, row_count: u32) -> Result<Vec<T>, std::io::Error> {
    let mut rows = Vec::new();
    for _ in 0..row_count {
        rows.push(T::read_from(buffer)?);
    }
    Ok(rows)
}

fn read_single_row<T: TableRow>(buffer: &mut PeParser, row_count: u32) -> Result<Option<T>, std::io::Error> {
    if row_count <= 0 {
        Ok(None)
    } else {
        Ok(Some(T::read_from(buffer)?))
    }
}

pub trait TableRow {
    fn read_from(buffer: &mut PeParser) -> Result<Self, std::io::Error> where Self: Sized;
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
    fn read_from(buffer: &mut PeParser) -> Result<AssemblyRow, std::io::Error> {
        Ok(AssemblyRow {
            hash_alg_id: buffer.read_u32()?,
            major_version: buffer.read_u16()?,
            minor_version: buffer.read_u16()?,
            build_number: buffer.read_u16()?,
            revision_number: buffer.read_u16()?,
            flags: AssemblyFlags::from(buffer.read_u32()? as u16),
            public_key: buffer.read_blob_index()?,
            name: buffer.read_string_index()?,
            culture: buffer.read_string_index()?,
        })
    }
}

/// # II.22.8 AssemblyRef : 0x23
/// 
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
    fn read_from(buffer: &mut PeParser) -> Result<AssemblyRefRow, std::io::Error> {
        Ok(AssemblyRefRow {
            major_version: buffer.read_u16()?,
            minor_version: buffer.read_u16()?,
            build_number: buffer.read_u16()?,
            revision_number: buffer.read_u16()?,
            flags: AssemblyFlags::from(buffer.read_u32()? as u16),
            public_key_or_token: buffer.read_blob_index()?,
            name: buffer.read_string_index()?,
            culture: buffer.read_string_index()?,
            hash_value: buffer.read_blob_index()?,
        })
    }
}

/// # II.22.8 ClassLayout : 0x0F
/// 
/// [...]
/// 
/// The ClassLayout table has the following columns: 
/// * *PackingSize* (a 2-byte constant)
/// * *ClassSize* (a 4-byte constant)
/// * *Parent* (an index into the TypeDef table)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClassLayoutRow {
    pub packing_size: u16,
    pub class_size: u32,
    pub parent: CodedIndex,
}

impl TableRow for ClassLayoutRow {
    fn read_from(buffer: &mut PeParser) -> Result<ClassLayoutRow, std::io::Error> {
        Ok(ClassLayoutRow {
            packing_size: buffer.read_u16()?,
            class_size: buffer.read_u32()?,
            parent: buffer.read_table_index(TableKind::TypeDef)?,
        })
    }
}

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
    fn read_from(buffer: &mut PeParser) -> Result<ConstantRow, std::io::Error> {
        Ok(ConstantRow {
            type_: buffer.read_u16()?,
            parent: buffer.read_coded_index(CodedIndexTag::HasConstant)?,
            value: buffer.read_blob_index()?,
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
    fn read_from(buffer: &mut PeParser) -> Result<CustomAttributeRow, std::io::Error> {
        Ok(CustomAttributeRow {
            parent: buffer.read_coded_index(CodedIndexTag::HasCustomAttribute)?,
            type_: buffer.read_coded_index(CodedIndexTag::CustomAttributeType)?,
            value: buffer.read_blob_index()?,
        })
    }
}

/// # II.22.11 DeclSecurity : 0x0E
/// 
/// [...]
/// 
/// The *DeclSecurity* table has the following columns: 
/// * *Action* (a 2-byte value)
/// * *Parent* (an index into the *TypeDef*, *MethodDef*, or *Assembly* table; more precisely, a HasDeclSecurity  (§II.24.2.6) coded index)
/// * *PermissionSet* (an index into the Blob heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeclSecurityRow {
    pub action: u16,
    pub parent: CodedIndex,
    pub permission_set: BlobIndex,
}

impl TableRow for DeclSecurityRow {
    fn read_from(buffer: &mut PeParser) -> Result<DeclSecurityRow, std::io::Error> {
        Ok(DeclSecurityRow {
            action: buffer.read_u16()?,
            parent: buffer.read_coded_index(CodedIndexTag::HasDeclSecurity)?,
            permission_set: buffer.read_blob_index()?,
        })
    }
}

/// # II.22.12 EventMap : 0x12
/// 
/// The *EventMap* table has the following columns:
/// * *Parent* (an index into the *TypeDef* table)
/// * *EventList* (an index into the *Event* table). It marks the first of a contiguous run of Events owned by this Type.
///   The run continues to the smaller of:
///     * the last row of the *Event* table
///     * the next run of Events, found by inspecting the *EventList* of the next row in the *EventMap* table
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventMapRow {
    pub parent: CodedIndex,
    pub event_list: CodedIndex,
}

impl TableRow for EventMapRow {
    fn read_from(buffer: &mut PeParser) -> Result<EventMapRow, std::io::Error> {
        Ok(EventMapRow {
            parent: buffer.read_table_index(TableKind::TypeDef)?,
            event_list: buffer.read_table_index(TableKind::Event)?,
        })
    }
}

/// # II.22.13 Event : 0x14 
/// 
/// [...]
/// 
/// The Event table has the following columns:
/// * *EventFlags* (a 2-byte bitmask of type *EventAttributes*, §II.23.1.4)
/// * *Name* (an index into the String heap)
/// * *EventType* (an index into a *TypeDef*, a *TypeRef*, or *TypeSpec* table; more precisely, a *TypeDefOrRef*  (§II.24.2.6) coded index)
///   (This corresponds to the Type of the Event; it is not the Type that owns this event.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventRow {
    pub event_flags: EventAttributes,
    pub name: StringIndex,
    pub event_type: CodedIndex,
}

impl TableRow for EventRow {
    fn read_from(buffer: &mut PeParser) -> Result<EventRow, std::io::Error> {
        Ok(EventRow {
            event_flags: EventAttributes::from(buffer.read_u16()?),
            name: buffer.read_string_index()?,
            event_type: buffer.read_coded_index(CodedIndexTag::TypeDefOrRef)?,
        })
    }
}

/// # II.22.14 ExportedType : 0x27 
/// 
/// [...]
/// 
/// The *ExportedType* table has the following columns: 
/// * *Flags* (a 4-byte bitmask of type *TypeAttributes*, §II.23.1.15)
/// * *TypeDefId* (a 4-byte index into a TypeDef table of another module in this Assembly).  
///    This column is used as a hint only.  If the entry in the target TypeDef table matches 
///    the TypeName and TypeNamespace entries in this table, resolution has succeeded.  
///    But if there is a mismatch, the CLI shall fall back to a search of the target TypeDef 
///    table. Ignored and should be zero if Flags has IsTypeForwarder set.
/// * *TypeName* (an index into the String heap)
/// * *TypeNamespace* (an index into the String heap)
/// * *Implementation* This is an index (more precisely, an Implementation (§II.24.2.6) coded index) into either of the following tables:
///      * *File* table, where that entry says which module in the current assembly holds the *TypeDef*
///      * *ExportedType*  table, where that entry is the enclosing Type of the current nested Type
///      * *AssemblyRef* table, where that entry says in which assembly the type may now be found (Flags must have the IsTypeForwarder flag set).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExportedTypeRow {
    pub flags: TypeAttributes,
    pub type_def_id: u32,
    pub type_name: StringIndex,
    pub type_namespace: StringIndex,
    pub implementation: CodedIndex,
}

impl TableRow for ExportedTypeRow {
    fn read_from(buffer: &mut PeParser) -> Result<ExportedTypeRow, std::io::Error> {
        Ok(ExportedTypeRow {
            flags: TypeAttributes::from(buffer.read_u32()?),
            type_def_id: buffer.read_u32()?,
            type_name: buffer.read_string_index()?,
            type_namespace: buffer.read_string_index()?,
            implementation: buffer.read_coded_index(CodedIndexTag::Implementation)?,
        })
    }
}

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
    fn read_from(buffer: &mut PeParser) -> Result<FieldRow, std::io::Error> {
        Ok(FieldRow {
            flags: FieldAttributes::from(buffer.read_u16()?),
            name: buffer.read_string_index()?,
            signature: buffer.read_blob_index()?,
        })
    }
}

/// # II.22.16 FieldLayout : 0x10
/// 
/// The *FieldLayout* table has the following columns:
/// * *Offset* (a 4-byte constant)
/// * *Field* (an index into the *Field* table)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldLayoutRow {
    pub offset: u32,
    pub field: CodedIndex,
}

impl TableRow for FieldLayoutRow {
    fn read_from(buffer: &mut PeParser) -> Result<FieldLayoutRow, std::io::Error> {
        Ok(FieldLayoutRow {
            offset: buffer.read_u32()?,
            field: buffer.read_table_index(TableKind::Field)?,
        })
    }
}

/// # II.22.17 FieldMarshal : 0x0D
/// 
/// [...]
/// 
/// The *FieldMarshal* table has the following columns:
/// * *Parent* (an index into *Field* or *Param* table; more precisely, a *HasFieldMarshal* (§II.24.2.6) coded index)
/// * *NativeType* (an index into the Blob heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldMarshalRow {
    pub parent: CodedIndex,
    pub native_type: BlobIndex,
}

impl TableRow for FieldMarshalRow {
    fn read_from(buffer: &mut PeParser) -> Result<FieldMarshalRow, std::io::Error> {
        Ok(FieldMarshalRow {
            parent: buffer.read_coded_index(CodedIndexTag::HasFieldMarshal)?,
            native_type: buffer.read_blob_index()?,
        })
    }
}

/// # II.22.18 FieldRVA : 0x1D
/// 
/// The *FieldRVA* table has the following columns:
/// * *RVA* (a 4-byte constant)
/// * *Field* (an index into the *Field* table)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldRVARow {
    pub rva: u32,
    pub field: CodedIndex,
}

impl TableRow for FieldRVARow {
    fn read_from(buffer: &mut PeParser) -> Result<FieldRVARow, std::io::Error> {
        Ok(FieldRVARow {
            rva: buffer.read_u32()?,
            field: buffer.read_table_index(TableKind::Field)?,
        })
    }
    
}

/// # II.22.19 File : 0x26
/// 
/// The *File* table has the following columns:
/// * *Flags* (a 4-byte bitmask of type *FileAttributes*, §II.23.1.6)
/// * *Name* (an index into the String heap)
/// * *HashValue* (an index into the Blob heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FileRow {
    pub flags: FileAttributes,
    pub name: StringIndex,
    pub hash_value: BlobIndex,
}

impl TableRow for FileRow {
    fn read_from(buffer: &mut PeParser) -> Result<FileRow, std::io::Error> {
        Ok(FileRow {
            flags: FileAttributes::from(buffer.read_u32()?),
            name: buffer.read_string_index()?,
            hash_value: buffer.read_blob_index()?,
        })
    }
    
}

/// # II.22.20 GenericParam : 0x2A 
/// 
/// The *GenericParam* table has the following columns:
/// * *Number* (the 2-byte index of the generic parameter, numbered left-to-right, from zero)
/// * *Flags* (a 2-byte bitmask of type *GenericParamAttributes*, §II.23.1.7)
/// * *Owner* (an index into the *TypeDef* or *MethodDef* table, specifying the Type or Method to
///   which this generic parameter applies; more precisely, a *TypeOrMethodDef* (§II.24.2.6) coded index)
/// * *Name* (a non-null index into the String heap, giving the name for the generic parameter.
///   This is purely descriptive and is used only by source language compilers and by Reflection)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GenericParamRow {
    pub number: u16,
    pub flags: GenericParamAttributes,
    pub owner: CodedIndex,
    pub name: StringIndex,
}

impl TableRow for GenericParamRow {
    fn read_from(buffer: &mut PeParser) -> Result<GenericParamRow, std::io::Error> {
        Ok(GenericParamRow {
            number: buffer.read_u16()?,
            flags: GenericParamAttributes::from(buffer.read_u16()?),
            owner: buffer.read_coded_index(CodedIndexTag::TypeOrMethodDef)?,
            name: buffer.read_string_index()?,
        })
    }
}

/// # II.22.21 GenericParamConstraint : 0x2C
/// 
/// The *GenericParamConstraint* table has the following columns: 
/// * *Owner* (an index into the *GenericParam* table, specifying to which generic parameter this row refers)
/// * *Constraint* (an index into the *TypeDef*, *TypeRef*, or *TypeSpec* tables, specifying from which class this generic parameter is constrained to derive;
///   or which interface this generic parameter is constrained to implement; more precisely, a *TypeDefOrRef* (§II.24.2.6) coded index)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GenericParamConstraintRow {
    pub owner: CodedIndex,
    pub constraint: CodedIndex,
}

impl TableRow for GenericParamConstraintRow {
    fn read_from(buffer: &mut PeParser) -> Result<GenericParamConstraintRow, std::io::Error> {
        Ok(GenericParamConstraintRow {
            owner: buffer.read_table_index(TableKind::GenericParam)?,
            constraint: buffer.read_coded_index(CodedIndexTag::TypeDefOrRef)?,
        })
    }
}

/// # II.22.22 ImplMap : 0x1C
///
/// [...]
/// 
/// The *ImplMap* table has the following columns:
/// * *MappingFlags* (a 2-byte bitmask of type *PInvokeAttributes*, §23.1.8)
/// * *MemberForwarded* (an index into the *Field* or *MethodDef* table; more precisely, a *MemberForwarded* (§II.24.2.6) coded index).
///   However, it only ever indexes the *MethodDef* table, since *Field* export is not supported. 
/// * *ImportName* (an index into the String heap)
/// * *ImportScope* (an index into the *ModuleRef* table)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImplMapRow {
    pub mapping_flags: PInvokeAttributes,
    pub member_forwarded: CodedIndex,
    pub import_name: StringIndex,
    pub import_scope: CodedIndex,
}

impl TableRow for ImplMapRow {
    fn read_from(buffer: &mut PeParser) -> Result<ImplMapRow, std::io::Error> {
        Ok(ImplMapRow {
            mapping_flags: PInvokeAttributes::from(buffer.read_u16()?),
            member_forwarded: buffer.read_coded_index(CodedIndexTag::MemberForwarded)?,
            import_name: buffer.read_string_index()?,
            import_scope: buffer.read_table_index(TableKind::ModuleRef)?,
        })
    }
}

/// # II.22.23 InterfaceImpl : 0x09
/// 
/// The *InterfaceImpl* table has the following columns:
/// * *Class* (an index into the *TypeDef* table)
/// * *Interface* (an index into the *TypeDef*, *TypeRef*, or *TypeSpec* table; more precisely, a *TypeDefOrRef*  (§II.24.2.6) coded index) 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterfaceImplRow {
    pub class: CodedIndex,
    pub interface: CodedIndex,
}

impl TableRow for InterfaceImplRow {
    fn read_from(buffer: &mut PeParser) -> Result<InterfaceImplRow, std::io::Error> {
        Ok(InterfaceImplRow {
            class: buffer.read_table_index(TableKind::TypeDef)?,
            interface: buffer.read_coded_index(CodedIndexTag::TypeDefOrRef)?,
        })
    }
}

/// # II.22.24 ManifestResource : 0x28 
/// 
/// The *ManifestResource* table has the following columns:
/// * *Offset* (a 4-byte constant)
/// * *Flags* (a 4-byte bitmask of type *ManifestResourceAttributes*, §II.23.1.9)
/// * *Name* (an index into the String heap)
/// * *Implementation* (an index into a *File* table, a *AssemblyRef* table, or null; more precisely, an *Implementation* (§II.24.2.6) coded index) 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManifestResourceRow {
    pub offset: u32,
    pub flags: ManifestResourceAttributes,
    pub name: StringIndex,
    pub implementation: CodedIndex,
}

impl TableRow for ManifestResourceRow {
    fn read_from(buffer: &mut PeParser) -> Result<ManifestResourceRow, std::io::Error> {
        Ok(ManifestResourceRow {
            offset: buffer.read_u32()?,
            flags: ManifestResourceAttributes::from(buffer.read_u32()?),
            name: buffer.read_string_index()?,
            implementation: buffer.read_coded_index(CodedIndexTag::Implementation)?,
        })
    }
}

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
    fn read_from(buffer: &mut PeParser) -> Result<MemberRefRow, std::io::Error> {
        Ok(MemberRefRow {
            class: buffer.read_coded_index(CodedIndexTag::MemberRefParent)?,
            name: buffer.read_string_index()?,
            signature: buffer.read_blob_index()?,
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
    fn read_from(buffer: &mut PeParser) -> Result<MethodDefRow, std::io::Error> {
        Ok(MethodDefRow {
            rva: buffer.read_u32()?,
            impl_flags: MethodImplAttributes::from(buffer.read_u16()?),
            flags: MethodAttributes::from(buffer.read_u16()?),
            name: buffer.read_string_index()?,
            signature: buffer.read_blob_index()?,
            param_list: buffer.read_table_index(TableKind::Param)?,
        })
    }
}

/// # II.22.27 MethodImpl : 0x19
/// 
/// [...]
/// 
/// The *MethodImpl* table has the following columns: 
/// * *Class* (an index into the *TypeDef* table)
/// * *MethodBody* (an index into the *MethodDef* or *MemberRef* table; more precisely, a *MethodDefOrRef* (§II.24.2.6) coded index)
/// * *MethodDeclaration* (an index into the *MethodDef* or *MemberRef* table; more precisely, a *MethodDefOrRef* (§II.24.2.6) coded index)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MethodImplRow {
    pub class: CodedIndex,
    pub method_body: CodedIndex,
    pub method_declaration: CodedIndex,
}

impl TableRow for MethodImplRow {
    fn read_from(buffer: &mut PeParser) -> Result<MethodImplRow, std::io::Error> {
        Ok(MethodImplRow {
            class: buffer.read_table_index(TableKind::TypeDef)?,
            method_body: buffer.read_coded_index(CodedIndexTag::MethodDefOrRef)?,
            method_declaration: buffer.read_coded_index(CodedIndexTag::MethodDefOrRef)?,
        })
    }
}

/// # II.22.28 MethodSemantics : 0x18
/// 
/// The *MethodSemantics* table has the following columns: 
/// * *Semantics* (a 2-byte bitmask of type *MethodSemanticsAttributes*, §II.23.1.12)
/// * *Method* (an index into the *MethodDef* table)
/// * *Association* (an index into the *Event* or *Property* table; more precisely, a *HasSemantics* (§II.24.2.6) coded index)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MethodSemanticsRow {
    pub semantics: MethodSemanticsAttributes,
    pub method: CodedIndex,
    pub association: CodedIndex,
}

impl TableRow for MethodSemanticsRow {
    fn read_from(buffer: &mut PeParser) -> Result<MethodSemanticsRow, std::io::Error> {
        Ok(MethodSemanticsRow {
            semantics: MethodSemanticsAttributes::from(buffer.read_u16()?),
            method: buffer.read_table_index(TableKind::MethodDef)?,
            association: buffer.read_coded_index(CodedIndexTag::HasSemantics)?,
        })
    }
}

/// # II.22.29 MethodSpec : 0x2B
/// 
/// The *MethodSpec* table has the following columns:
/// * *Method* (an index into the *MethodDef* or *MemberRef* table, specifying to which generic method this row refers; that is,
///   which generic method this row is an instantiation of; more precisely, a *MethodDefOrRef* (§II.24.2.6) coded index)
/// * *Instantiation* (an index into the Blob heap (§II.23.2.15), holding the signature of this instantiation)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MethodSpecRow {
    pub method: CodedIndex,
    pub instantiation: BlobIndex,
}

impl TableRow for MethodSpecRow {
    fn read_from(buffer: &mut PeParser) -> Result<MethodSpecRow, std::io::Error> {
        Ok(MethodSpecRow {
            method: buffer.read_coded_index(CodedIndexTag::MethodDefOrRef)?,
            instantiation: buffer.read_blob_index()?,
        })
    }
}

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
    fn read_from(buffer: &mut PeParser) -> Result<ModuleRow, std::io::Error> {
        Ok(ModuleRow {
            generation: buffer.read_u16()?,
            name: buffer.read_string_index()?,
            mvid: buffer.read_guid_index()?,
            enc_id: buffer.read_guid_index()?,
            enc_base_id: buffer.read_guid_index()?,
        })
    }
}

/// # II.22.31 ModuleRef : 0x1A
///
/// The *ModuleRef* table has the following columns:
/// * *Name* (an index into the String heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleRefRow {
    pub name: StringIndex,
}

impl TableRow for ModuleRefRow {
    fn read_from(buffer: &mut PeParser) -> Result<ModuleRefRow, std::io::Error> {
        Ok(ModuleRefRow {
            name: buffer.read_string_index()?,
        })
    }
}

/// # II.22.32 NestedClass : 0x29
/// 
/// The *NestedClass* table has the following columns: 
/// * *NestedClass* (an index into the *TypeDef* table) 
/// * *EnclosingClass* (an index into the *TypeDef* table)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NestedClassRow {
    pub nested_class: CodedIndex,
    pub enclosing_class: CodedIndex,
}

impl TableRow for NestedClassRow {
    fn read_from(buffer: &mut PeParser) -> Result<NestedClassRow, std::io::Error> {
        Ok(NestedClassRow {
            nested_class: buffer.read_table_index(TableKind::TypeDef)?,
            enclosing_class: buffer.read_table_index(TableKind::TypeDef)?,
        })
    }
}

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
    fn read_from(buffer: &mut PeParser) -> Result<ParamRow, std::io::Error> {
        Ok(ParamRow {
            flags: ParamAttributes::from(buffer.read_u16()?),
            sequence: buffer.read_u16()?,
            name: buffer.read_string_index()?,
        })
    }
}

/// # II.22.34 Property : 0x17
/// 
/// [...]
/// 
/// The *Property* ( 0x17 ) table has the following columns:
/// * *Flags* (a 2-byte bitmask of type PropertyAttributes, §II.23.1.14)
/// * *Name* (an index into the String heap)
/// * *Type* (an index into the Blob heap)
///   (The name of this column is misleading. It does not index a *TypeDef* or *TypeRef* table—instead it indexes the signature in the Blob heap of the Property)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PropertyRow {
    pub flags: PropertyAttributes,
    pub name: StringIndex,
    pub type_: BlobIndex,
}

impl TableRow for PropertyRow {
    fn read_from(buffer: &mut PeParser) -> Result<PropertyRow, std::io::Error> {
        Ok(PropertyRow {
            flags: PropertyAttributes::from(buffer.read_u16()?),
            name: buffer.read_string_index()?,
            type_: buffer.read_blob_index()?,
        })
    }
}

/// # II.22.35 PropertyMap : 0x15
/// 
/// The *PropertyMap* table has the following columns:
/// * *Parent* (an index into the *TypeDef* table) 
/// * *PropertyList* (an index into the *Property* table). It marks the first of a contiguous run of Properties owned by *Parent*.
///   The run continues to the smaller of:
///     * the last row of the *Property* table
///     * the next run of Properties, found by inspecting the *PropertyList* of the next row in this *PropertyMap* table
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PropertyMapRow {
    pub parent: CodedIndex,
    pub property_list: CodedIndex,
}

impl TableRow for PropertyMapRow {
    fn read_from(buffer: &mut PeParser) -> Result<PropertyMapRow, std::io::Error> {
        Ok(PropertyMapRow {
            parent: buffer.read_table_index(TableKind::TypeDef)?,
            property_list: buffer.read_table_index(TableKind::Property)?,
        })
    }
}

/// # II.22.36 StandAloneSig : 0x11
/// 
/// [...]
/// 
/// The *StandAloneSig* table has the following column:
///  * *Signature* (an index into the Blob heap)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StandAloneSigRow {
    pub signature: BlobIndex,
}

impl TableRow for StandAloneSigRow {
    fn read_from(buffer: &mut PeParser) -> Result<StandAloneSigRow, std::io::Error> {
        Ok(StandAloneSigRow {
            signature: buffer.read_blob_index()?,
        })
    }
}

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
    fn read_from(buffer: &mut PeParser) -> Result<TypeDefRow, std::io::Error> {
        Ok(TypeDefRow {
            flags: TypeAttributes::from(buffer.read_u32()?),
            type_name: buffer.read_string_index()?,
            type_namespace: buffer.read_string_index()?,
            extends: buffer.read_coded_index(CodedIndexTag::TypeDefOrRef)?,
            field_list: buffer.read_table_index(TableKind::Field)?,
            method_list: buffer.read_table_index(TableKind::MethodDef)?,
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
    fn read_from(buffer: &mut PeParser) -> Result<TypeRefRow, std::io::Error> {
        Ok(TypeRefRow {
            resolution_scope: buffer.read_coded_index(CodedIndexTag::ResolutionScope)?,
            type_name: buffer.read_string_index()?,
            type_namespace: buffer.read_string_index()?,
        })
    }
}

/// # II.22.39 TypeSpec : 0x1B
/// 
///  [...]
/// 
/// The *TypeSpec* table has the following column: 
/// * *Signature* (index into the Blob heap, where the blob is formatted as specified in §II.23.2.14)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeSpecRow {
    pub signature: BlobIndex,
}

impl TableRow for TypeSpecRow {
    fn read_from(buffer: &mut PeParser) -> Result<TypeSpecRow, std::io::Error> {
        Ok(TypeSpecRow {
            signature: buffer.read_blob_index()?,
        })
    }
}
