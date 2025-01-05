
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableKind {
    /// # II.22.2 Assembly : 0x20
    /// See [`AssemblyRow`]
    Assembly,
    /// # II.22.3 AssemblyOS : 0x22
    /// [...]
    /// 
    /// It shall be ignored by the CLI. 
    AssemblyOS,
    /// # II.22.4 AssemblyProcessor : 0x21
    /// [...]
    /// It should be ignored by the CLI.
    AssemblyProcessor,
    /// # II.22.5 AssemblyRef : 0x23
    /// See [`AssemblyRefRow`]
    AssemblyRef,
    /// # II.22.6 AssemblyRefOS : 0x25
    /// [...]
    /// They should be ignored by the CLI.
    AssemblyRefOS,
    /// # II.22.7 AssemblyRefProcessor : 0x24
    /// [...]
    /// They should be ignored by the CLI.
    AssemblyRefProcessor,
    /// # II.22.8 ClassLayout : 0x0F
    /// See [`ClassLayoutRow`]
    ClassLayout,
    /// II.22.9 Constant : 0x0B
    /// See [`ConstantRow`]
    Constant,
    /// # II.22.10 CustomAttribute : 0x0C
    CustomAttribute,
    /// # II.22.11 DeclSecurity : 0x0E
    /// See [`DeclSecurityRow`]
    DeclSecurity,
    /// # II.22.12 EventMap : 0x12
    /// See [`EventMapRow`]
    EventMap, 
    /// # II.22.13 Event : 0x14
    /// See [`EventRow`]
    Event,
    /// # II.22.14 ExportedType : 0x27
    /// See [`ExportedTypeRow`]
    ExportedType,
    /// # II.22.15 Field : 0x04
    /// See [`FieldRow`]
    Field,
    /// # II.22.16 FieldLayout : 0x10
    /// See [`FieldLayoutRow`]
    FieldLayout, 
    /// # II.22.17 FieldMarshal : 0x0D
    /// See [`FieldMarshalRow`]
    FieldMarshal,
    /// # II.22.18 FieldRVA : 0x1D
    /// See [`FieldRVARow`]
    FieldRVA, 
    /// # II.22.19 File : 0x26
    /// See [`FileRow`]
    File,
    /// # II.22.20 GenericParam : 0x2A
    /// See [`GenericParamRow`]
    GenericParam,
    /// # II.22.21 GenericParamConstraint : 0x2C
    /// See [`GenericParamConstraintRow`]
    GenericParamConstraint,
    /// # II.22.22 ImplMap : 0x1C
    /// See [`ImplMapRow`]
    ImplMap, 
    /// # II.22.23 InterfaceImpl : 0x09
    /// See [`InterfaceImplRow`]
    InterfaceImpl,
    /// # II.22.24 ManifestResource : 0x28
    /// See [`ManifestResourceRow`]
    ManifestResource,
    /// # II.22.25 MemberRef : 0x0A 
    /// See [`MemberRefRow`]
    MemberRef,
    /// # II.22.26 MethodDef : 0x06
    /// See [`MethodDefRow`]
    MethodDef,
    /// # II.22.27 MethodImpl : 0x19
    /// See [`MethodImplRow`]
    MethodImpl,
    /// # II.22.28 MethodSemantics : 0x18
    /// See [`MethodSemanticsRow`]
    MethodSemantics,
    /// # II.22.29 MethodSpec : 0x2B
    /// See [`MethodSpecRow`]
    MethodSpec,
    /// # II.22.30 Module : 0x00 
    /// See [`ModuleRow`]
    Module,
    /// # II.22.31 ModuleRef : 0x1A
    /// See [`ModuleRefRow`]
    ModuleRef,
    /// # II.22.32 NestedClass : 0x29
    /// See [`NestedClassRow`]
    NestedClass,
    /// II.22.33 Param : 0x08
    /// See [`ParamRow`]
    Param,
    /// # II.22.34 Property : 0x17
    /// See [`PropertyRow`]
    Property,
    /// # II.22.35 PropertyMap : 0x15
    /// See [`PropertyMapRow`]
    PropertyMap,
    /// # II.22.36 StandAloneSig : 0x11
    /// See [`StandAloneSigRow`]
    StandAloneSig,
    /// # II.22.37 TypeDef : 0x02
    /// See [`TypeDefRow`]
    TypeDef,
    /// # II.22.38 TypeRef : 0x01
    /// See [`TypeRefRow`]
    TypeRef,
    /// # II.22.39 TypeSpec : 0x1B
    /// See [`TypeSpecRow`]
    TypeSpec,
}

impl TableKind {
    pub const NUM_TABLES: usize = 45;

    pub fn from_bitmask(bitmask: u64) -> Vec<TableKind> {
        let mut kinds = Vec::new();
        for i in 0..64 {
            if (bitmask & (1 << i)) != 0 {
                kinds.push(TableKind::from_u32(i).unwrap());
            }
        }
        kinds
    }

    pub fn from_u32(index: u32) -> Option<TableKind> {
        match index {
            0x20 => Some(TableKind::Assembly),
            0x22 => Some(TableKind::AssemblyOS),
            0x21 => Some(TableKind::AssemblyProcessor),
            0x23 => Some(TableKind::AssemblyRef),
            0x25 => Some(TableKind::AssemblyRefOS),
            0x24 => Some(TableKind::AssemblyRefProcessor),
            0x0f => Some(TableKind::ClassLayout),
            0x0b => Some(TableKind::Constant),
            0x0c => Some(TableKind::CustomAttribute),
            0x0e => Some(TableKind::DeclSecurity),
            0x12 => Some(TableKind::EventMap),
            0x14 => Some(TableKind::Event),
            0x27 => Some(TableKind::ExportedType),
            0x04 => Some(TableKind::Field),
            0x10 => Some(TableKind::FieldLayout),
            0x0d => Some(TableKind::FieldMarshal),
            0x1d => Some(TableKind::FieldRVA),
            0x26 => Some(TableKind::File),
            0x2a => Some(TableKind::GenericParam),
            0x2c => Some(TableKind::GenericParamConstraint),
            0x1c => Some(TableKind::ImplMap),
            0x09 => Some(TableKind::InterfaceImpl),
            0x28 => Some(TableKind::ManifestResource),
            0x0a => Some(TableKind::MemberRef),
            0x06 => Some(TableKind::MethodDef),
            0x19 => Some(TableKind::MethodImpl),
            0x18 => Some(TableKind::MethodSemantics),
            0x2b => Some(TableKind::MethodSpec),
            0x00 => Some(TableKind::Module),
            0x1a => Some(TableKind::ModuleRef),
            0x29 => Some(TableKind::NestedClass),
            0x08 => Some(TableKind::Param),
            0x17 => Some(TableKind::Property),
            0x15 => Some(TableKind::PropertyMap),
            0x11 => Some(TableKind::StandAloneSig),
            0x02 => Some(TableKind::TypeDef),
            0x01 => Some(TableKind::TypeRef),
            0x1b => Some(TableKind::TypeSpec),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            TableKind::Assembly => 0x20,
            TableKind::AssemblyOS => 0x22,
            TableKind::AssemblyProcessor => 0x21,
            TableKind::AssemblyRef => 0x23,
            TableKind::AssemblyRefOS => 0x25,
            TableKind::AssemblyRefProcessor => 0x24,
            TableKind::ClassLayout => 0x0f,
            TableKind::Constant => 0x0b,
            TableKind::CustomAttribute => 0x0c,
            TableKind::DeclSecurity => 0x0e,
            TableKind::EventMap => 0x12,
            TableKind::Event => 0x14,
            TableKind::ExportedType => 0x27,
            TableKind::Field => 0x04,
            TableKind::FieldLayout => 0x10,
            TableKind::FieldMarshal => 0x0d,
            TableKind::FieldRVA => 0x1d,
            TableKind::File => 0x26,
            TableKind::GenericParam => 0x2a,
            TableKind::GenericParamConstraint => 0x2c,
            TableKind::ImplMap => 0x1c,
            TableKind::InterfaceImpl => 0x09,
            TableKind::ManifestResource => 0x28,
            TableKind::MemberRef => 0x0a,
            TableKind::MethodDef => 0x06,
            TableKind::MethodImpl => 0x19,
            TableKind::MethodSemantics => 0x18,
            TableKind::MethodSpec => 0x2b,
            TableKind::Module => 0x00,
            TableKind::ModuleRef => 0x1a,
            TableKind::NestedClass => 0x29,
            TableKind::Param => 0x08,
            TableKind::Property => 0x17,
            TableKind::PropertyMap => 0x15,
            TableKind::StandAloneSig => 0x11,
            TableKind::TypeDef => 0x02,
            TableKind::TypeRef => 0x01,
            TableKind::TypeSpec => 0x1b,
        }
    }

    pub fn read(self, buffer: &mut Buffer, context: &TableDecodeContext) -> Result<CodedIndex, std::io::Error> {
        if context.get_table_index_size(self) == 2 {
            return Ok(CodedIndex::from(self, buffer.read_u16::<LittleEndian>()? as u32));
        }
        else {
            return Ok(CodedIndex::from(self, buffer.read_u32::<LittleEndian>()?));
        }
    }
}