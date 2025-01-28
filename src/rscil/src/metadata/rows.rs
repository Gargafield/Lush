
use super::*;

#[macro_export]
macro_rules! cast_row {
    (Some($row:ty), $expr:expr) => {
        if let Some(row) = $expr {
            <$row>::from_row(row)
        }
        else {
            None
        }
    };

    ($row:path, $expr:expr) => {
        match $expr {
            $row(x) => x,
            _ => panic!("Failed to cast row to {}", stringify!($row))
        }
    };
}

macro_rules! define_row {
    {$($path:path)*} => {
        
    };
}

define_row!{u8 TableKind::Assembly}

macro_rules! define_rows {
    {
    #[$($attr:meta)*]
    $visibility:vis enum $name:ident {
        $(
        $(#[$($enum_attr:meta)*])*
        $enum_name:ident {
            $($prop_vis:vis $prop_name:ident: $($prop_type:ident$(::)?)*,)*
        })*
    }} => {
        #[$($attr)*]
        $visibility enum $name {
            $($enum_name($enum_name),)*
        }

        impl $name {
            pub fn read(buffer: &mut Buffer, kind: TableKind, context: &TableDecodeContext) -> Result<Self, std::io::Error> {
                match kind {
                    $(
                        TableKind::$enum_name => Ok($name::$enum_name($enum_name::decode(context, buffer)?)),
                    )*
                    _ => panic!("Invalid table kind: {:?}", kind)
                }
            }
        }

        $(
            $(#[$($enum_attr)*])*
            #[derive(Debug, Clone, Copy)]
            $visibility struct $enum_name {
                pub index: u32,
                $($prop_vis $prop_name: define_rows!(@type $($prop_type)*),)*
            }

            impl $enum_name {
                pub fn from_row(row: &$name) -> Option<&Self> {
                    match row {
                        $name::$enum_name(row) => Some(row),
                        _ => None
                    }
                }
            }

            impl TableDecode for $enum_name {
                type Output = Self;

                fn decode(context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self, std::io::Error> {
                    Ok($enum_name {
                        index: context.get_index(TableKind::$enum_name),
                        $($prop_name: define_rows!(@decode $($prop_type)* [context, buffer]),)*
                    })
                }
            }
        )*
    };
    (@type $enum:ident $name:ident) => {
        <$enum as TableEnumDecode>::Output
    };
    (@type $type:ty) => {
        <$type as TableDecode>::Output
    };
    (@decode $enum:ident $name:ident [$context:expr, $buffer:expr]) => {
        $enum::$name.decode($context, $buffer)?
    };
    (@decode $type:ty [$context:expr, $buffer:expr]) => {
        <$type>::decode($context, $buffer)?
    };
}



define_rows!{
    #[derive(Debug, Clone, Copy)]
    pub enum Row {
        /// # II.22.2 Assembly : 0x20
        /// The *Assembly* table has the following columns: 
        /// * *HashAlgId* (a 4-byte constant of type AssemblyHashAlgorithm, §II.23.1.1)
        /// * *MajorVersion*, *MinorVersion*, *BuildNumber*, *RevisionNumber* (each being 2-byte constants) 
        /// * *Flags* (a 4-byte bitmask of type AssemblyFlags, §II.23.1.2)
        /// * *PublicKey* (an index into the Blob heap)
        /// * *Name* (an index into the String heap)
        /// * *Culture* (an index into the String heap)
        Assembly {
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

        /// # II.22.8 AssemblyRef : 0x23
        /// 
        /// The *AssemblyRef* table has the following columns: 
        /// * *MajorVersion*, *MinorVersion*, *BuildNumber*, *RevisionNumber* (each being 2-byte constants)
        /// * *Flags* (a 4-byte bitmask of type AssemblyFlags, §II.23.1.2)
        /// * *PublicKeyOrToken* (an index into the Blob heap, indicating the public key or token that identifies the author of this Assembly)
        /// * *Name* (an index into the String heap)
        /// * *Culture* (an index into the String heap)
        /// * *HashValue* (an index into the Blob heap)
        AssemblyRef {
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

        /// # II.22.8 ClassLayout : 0x0F
        /// 
        /// [...]
        /// 
        /// The ClassLayout table has the following columns: 
        /// * *PackingSize* (a 2-byte constant)
        /// * *ClassSize* (a 4-byte constant)
        /// * *Parent* (an index into the TypeDef table)
        ClassLayout {
            pub packing_size: u16,
            pub class_size: u32,
            pub parent: TableKind::TypeDef,
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
        Constant {
            pub type_: u16,
            pub parent: CodedIndexTag::HasConstant,
            pub value: BlobIndex,
        }

        /// # II.22.10 CustomAttribute : 0x0C
        /// 
        /// The *CustomAttribute* table has the following columns:
        /// 
        /// * *Parent* (an index into a metadata table that has an associated *HasCustomAttribute* (§II.24.2.6) coded index)
        /// * *Type* (an index into the *MethodDef* or *MemberRef* table; more precisely, a *CustomAttributeType* (§II.24.2.6) coded index). 
        /// * *Value* (an index into the Blob heap). 
        CustomAttribute {
            pub parent: CodedIndexTag::HasCustomAttribute,
            pub type_: CodedIndexTag::CustomAttributeType,
            pub value: BlobIndex,
        }

        /// # II.22.11 DeclSecurity : 0x0E
        /// 
        /// [...]
        /// 
        /// The *DeclSecurity* table has the following columns: 
        /// * *Action* (a 2-byte value)
        /// * *Parent* (an index into the *TypeDef*, *MethodDef*, or *Assembly* table; more precisely, a HasDeclSecurity  (§II.24.2.6) coded index)
        /// * *PermissionSet* (an index into the Blob heap)
        DeclSecurity {
            pub action: u16,
            pub parent: CodedIndexTag::HasDeclSecurity,
            pub permission_set: BlobIndex,
        }

        /// # II.22.12 EventMap : 0x12
        /// 
        /// The *EventMap* table has the following columns:
        /// * *Parent* (an index into the *TypeDef* table)
        /// * *EventList* (an index into the *Event* table). It marks the first of a contiguous run of Events owned by this Type.
        ///   The run continues to the smaller of:
        ///     * the last row of the *Event* table
        ///     * the next run of Events, found by inspecting the *EventList* of the next row in the *EventMap* table
        EventMap {
            pub parent: TableKind::TypeDef,
            pub event_list: TableKind::Event,
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
        Event {
            pub event_flags: EventAttributes,
            pub name: StringIndex,
            pub event_type: CodedIndexTag::TypeDefOrRef,
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
        ExportedType {
            pub flags: TypeAttributes,
            pub type_def_id: u32,
            pub type_name: StringIndex,
            pub type_namespace: StringIndex,
            pub implementation: CodedIndexTag::Implementation,
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
        Field {
            pub flags: FieldAttributes,
            pub name: StringIndex,
            pub signature: BlobIndex,
        }

        /// # II.22.16 FieldLayout : 0x10
        /// 
        /// The *FieldLayout* table has the following columns:
        /// * *Offset* (a 4-byte constant)
        /// * *Field* (an index into the *Field* table)
        FieldLayout {
            pub offset: u32,
            pub field: TableKind::Field,
        }

        /// # II.22.17 FieldMarshal : 0x0D
        /// 
        /// [...]
        /// 
        /// The *FieldMarshal* table has the following columns:
        /// * *Parent* (an index into *Field* or *Param* table; more precisely, a *HasFieldMarshal* (§II.24.2.6) coded index)
        /// * *NativeType* (an index into the Blob heap)
        FieldMarshal {
            pub parent: CodedIndexTag::HasFieldMarshal,
            pub native_type: BlobIndex,
        }

        /// # II.22.18 FieldRVA : 0x1D
        /// 
        /// The *FieldRVA* table has the following columns:
        /// * *RVA* (a 4-byte constant)
        /// * *Field* (an index into the *Field* table)
        FieldRVA {
            pub rva: u32,
            pub field: TableKind::Field,
        }

        /// # II.22.19 File : 0x26
        /// 
        /// The *File* table has the following columns:
        /// * *Flags* (a 4-byte bitmask of type *FileAttributes*, §II.23.1.6)
        /// * *Name* (an index into the String heap)
        /// * *HashValue* (an index into the Blob heap)
        File {
            pub flags: FileAttributes,
            pub name: StringIndex,
            pub hash_value: BlobIndex,
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
        GenericParam {
            pub number: u16,
            pub flags: GenericParamAttributes,
            pub owner: CodedIndexTag::TypeOrMethodDef,
            pub name: StringIndex,
        }

        /// # II.22.21 GenericParamConstraint : 0x2C
        /// 
        /// The *GenericParamConstraint* table has the following columns: 
        /// * *Owner* (an index into the *GenericParam* table, specifying to which generic parameter this row refers)
        /// * *Constraint* (an index into the *TypeDef*, *TypeRef*, or *TypeSpec* tables, specifying from which class this generic parameter is constrained to derive;
        ///   or which interface this generic parameter is constrained to implement; more precisely, a *TypeDefOrRef* (§II.24.2.6) coded index)
        GenericParamConstraint {
            pub owner: TableKind::GenericParam,
            pub constraint: CodedIndexTag::TypeDefOrRef,
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
        ImplMap {
            pub mapping_flags: PInvokeAttributes,
            pub member_forwarded: CodedIndexTag::MemberForwarded,
            pub import_name: StringIndex,
            pub import_scope: TableKind::ModuleRef,
        }

        /// # II.22.23 InterfaceImpl : 0x09
        /// 
        /// The *InterfaceImpl* table has the following columns:
        /// * *Class* (an index into the *TypeDef* table)
        /// * *Interface* (an index into the *TypeDef*, *TypeRef*, or *TypeSpec* table; more precisely, a *TypeDefOrRef*  (§II.24.2.6) coded index) 
        InterfaceImpl {
            pub class: TableKind::TypeDef,
            pub interface: CodedIndexTag::TypeDefOrRef,
        }

        /// # II.22.24 ManifestResource : 0x28 
        /// 
        /// The *ManifestResource* table has the following columns:
        /// * *Offset* (a 4-byte constant)
        /// * *Flags* (a 4-byte bitmask of type *ManifestResourceAttributes*, §II.23.1.9)
        /// * *Name* (an index into the String heap)
        /// * *Implementation* (an index into a *File* table, a *AssemblyRef* table, or null; more precisely, an *Implementation* (§II.24.2.6) coded index) 
        ManifestResource {
            pub offset: u32,
            pub flags: ManifestResourceAttributes,
            pub name: StringIndex,
            pub implementation: CodedIndexTag::Implementation,
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
        MemberRef {
            pub class: CodedIndexTag::MemberRefParent,
            pub name: StringIndex,
            pub signature: BlobIndex,
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
        MethodDef {
            pub rva: u32,
            pub impl_flags: MethodImplAttributes,
            pub flags: MethodAttributes,
            pub name: StringIndex,
            pub signature: BlobIndex,
            pub param_list: TableKind::Param,
        }

        /// # II.22.27 MethodImpl : 0x19
        /// 
        /// [...]
        /// 
        /// The *MethodImpl* table has the following columns: 
        /// * *Class* (an index into the *TypeDef* table)
        /// * *MethodBody* (an index into the *MethodDef* or *MemberRef* table; more precisely, a *MethodDefOrRef* (§II.24.2.6) coded index)
        /// * *MethodDeclaration* (an index into the *MethodDef* or *MemberRef* table; more precisely, a *MethodDefOrRef* (§II.24.2.6) coded index)
        MethodImpl {
            pub class: TableKind::TypeDef,
            pub method_body: CodedIndexTag::MethodDefOrRef,
            pub method_declaration: CodedIndexTag::MethodDefOrRef,
        }

        /// # II.22.28 MethodSemantics : 0x18
        /// 
        /// The *MethodSemantics* table has the following columns: 
        /// * *Semantics* (a 2-byte bitmask of type *MethodSemanticsAttributes*, §II.23.1.12)
        /// * *Method* (an index into the *MethodDef* table)
        /// * *Association* (an index into the *Event* or *Property* table; more precisely, a *HasSemantics* (§II.24.2.6) coded index)
        MethodSemantics {
            pub semantics: MethodSemanticsAttributes,
            pub method: TableKind::MethodDef,
            pub association: CodedIndexTag::HasSemantics,
        }

        /// # II.22.29 MethodSpec : 0x2B
        /// 
        /// The *MethodSpec* table has the following columns:
        /// * *Method* (an index into the *MethodDef* or *MemberRef* table, specifying to which generic method this row refers; that is,
        ///   which generic method this row is an instantiation of; more precisely, a *MethodDefOrRef* (§II.24.2.6) coded index)
        /// * *Instantiation* (an index into the Blob heap (§II.23.2.15), holding the signature of this instantiation)
        MethodSpec {
            pub method: CodedIndexTag::MethodDefOrRef,
            pub instantiation: BlobIndex,
        }

        /// # II.22.30 Module : 0x00
        /// 
        /// The *Module* table has the following columns:
        /// * *Generation* (a 2-byte value, reserved, shall be zero) 
        /// * *Name* (an index into the String heap)
        /// * *Mvid*  (an index into the Guid heap; simply a Guid used to distinguish between two versions of the same module)
        /// * *EncId* (an index into the Guid heap; reserved, shall be zero) 
        /// * *EncBaseId* (an index into the Guid heap; reserved, shall be zero)
        Module {
            pub generation: u16,
            pub name: StringIndex,
            pub mvid: GuidIndex,
            pub enc_id: GuidIndex,
            pub enc_base_id: GuidIndex,
        }

        /// # II.22.31 ModuleRef : 0x1A
        ///
        /// The *ModuleRef* table has the following columns:
        /// * *Name* (an index into the String heap)
        ModuleRef {
            pub name: StringIndex,
        }

        /// # II.22.32 NestedClass : 0x29
        /// 
        /// The *NestedClass* table has the following columns: 
        /// * *NestedClass* (an index into the *TypeDef* table) 
        /// * *EnclosingClass* (an index into the *TypeDef* table)
        NestedClass {
            pub nested_class: TableKind::TypeDef,
            pub enclosing_class: TableKind::TypeDef,
        }

        /// II.22.33 Param : 0x08
        /// 
        /// The *Param* table has the following columns: 
        /// * *Flags* (a 2-byte bitmask of type ParamAttributes, §II.23.1.13)
        /// * *Sequence* (a 2-byte constant)
        /// * *Name* (an index into the String heap)
        /// 
        /// Conceptually, every row in the Param table is owned by one, and only one, row in the *MethodDef* table.
        Param {
            pub flags: ParamAttributes,
            pub sequence: u16,
            pub name: StringIndex,
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
        Property {
            pub flags: PropertyAttributes,
            pub name: StringIndex,
            pub type_: BlobIndex,
        }

        /// # II.22.35 PropertyMap : 0x15
        /// 
        /// The *PropertyMap* table has the following columns:
        /// * *Parent* (an index into the *TypeDef* table) 
        /// * *PropertyList* (an index into the *Property* table). It marks the first of a contiguous run of Properties owned by *Parent*.
        ///   The run continues to the smaller of:
        ///     * the last row of the *Property* table
        ///     * the next run of Properties, found by inspecting the *PropertyList* of the next row in this *PropertyMap* table
        PropertyMap {
            pub parent: TableKind::TypeDef,
            pub property_list: TableKind::Property,
        }

        /// # II.22.36 StandAloneSig : 0x11
        /// 
        /// [...]
        /// 
        /// The *StandAloneSig* table has the following column:
        ///  * *Signature* (an index into the Blob heap)
        StandAloneSig {
            pub signature: BlobIndex,
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
        TypeDef {
            pub flags: TypeAttributes,
            pub type_name: StringIndex,
            pub type_namespace: StringIndex,
            pub extends: CodedIndexTag::TypeDefOrRef,
            pub field_list: TableKind::Field,
            pub method_list: TableKind::MethodDef,
        }

        /// # II.22.38 TypeRef : 0x01
        /// 
        /// The *TypeRef* table has the following columns:
        /// * *ResolutionScope* (an index into a *Module*, *ModuleRef*, *AssemblyRef* or *TypeRef* table, 
        ///   or null; more precisely, a *ResolutionScope* (§II.24.2.6) coded index)
        /// * *TypeName* (an index into the String heap)
        /// * *TypeNamespace* (an index into the String heap)
        TypeRef {
            pub resolution_scope: CodedIndexTag::ResolutionScope,
            pub type_name: StringIndex,
            pub type_namespace: StringIndex,
        }

        /// # II.22.39 TypeSpec : 0x1B
        /// 
        ///  [...]
        /// 
        /// The *TypeSpec* table has the following column: 
        /// * *Signature* (index into the Blob heap, where the blob is formatted as specified in §II.23.2.14)
        TypeSpec {
            pub signature: BlobIndex,
        }
    }
}

