
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableKind {
    /// # II.22.2 Assembly : 0x20
    /// See [`AssemblyRow`]
    Assembly,
    /// # II.22.8 AssemblyRef : 0x23
    /// See [`AssemblyRefRow`]
    AssemblyRef,
    /// II.22.9 Constant : 0x0B
    /// See [`ConstantRow`]
    Constant,
    /// # II.22.10 CustomAttribute : 0x0C
    CustomAttribute,
    /// # II.22.15 Field : 0x04
    /// See [`FieldRow`]
    Field,
    /// # II.22.20 GenericParam : 0x2A
    GenericParam, 
    /// # II.22.23 InterfaceImpl : 0x09
    InterfaceImpl,
    /// # II.22.24 ManifestResource : 0x28
    ManifestResource,
    /// # II.22.25 MemberRef : 0x0A 
    /// See [`MemberRefRow`]
    MemberRef,
    /// # II.22.26 MethodDef : 0x06
    /// See [`MethodDefRow`]
    MethodDef,
    /// # II.22.30 Module : 0x00 
    /// See [`ModuleRow`]
    Module,
    /// # II.22.32 NestedClass : 0x29
    NestedClass,
    /// II.22.33 Param : 0x08
    /// See [`ParamRow`]
    Param,
    /// # II.22.37 TypeDef : 0x02
    /// See [`TypeDefRow`]
    TypeDef,
    /// # II.22.38 TypeRef : 0x01
    /// See [`TypeRefRow`]
    TypeRef,
}

impl TableKind {
    pub const NUM_TABLES: usize = 45;

    pub fn from_bitmask(bitmask: u64) -> Vec<TableKind> {
        let mut kinds = Vec::new();
        for i in 0..64 {
            if (bitmask & (1 << i)) != 0 {
                kinds.push(TableKind::into(i));
            }
        }
        kinds
    }

    fn into(index: usize) -> TableKind {
        match index {
            0x20 => TableKind::Assembly,
            0x23 => TableKind::AssemblyRef,
            0x0b => TableKind::Constant,
            0x0c => TableKind::CustomAttribute,
            0x04 => TableKind::Field,
            0x2a => TableKind::GenericParam,
            0x09 => TableKind::InterfaceImpl,
            0x28 => TableKind::ManifestResource,
            0x0a => TableKind::MemberRef,
            0x06 => TableKind::MethodDef,
            0x00 => TableKind::Module,
            0x29 => TableKind::NestedClass,
            0x08 => TableKind::Param,
            0x02 => TableKind::TypeDef,
            0x01 => TableKind::TypeRef,
            _ => panic!("Unknown table kind: 0x{:02x}", index),
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            TableKind::Assembly => 0x20,
            TableKind::AssemblyRef => 0x23,
            TableKind::Constant => 0x0b,
            TableKind::CustomAttribute => 0x0c,
            TableKind::Field => 0x04,
            TableKind::GenericParam => 0x2a,
            TableKind::InterfaceImpl => 0x09,
            TableKind::ManifestResource => 0x28,
            TableKind::MemberRef => 0x0a,
            TableKind::MethodDef => 0x06,
            TableKind::Module => 0x00,
            TableKind::NestedClass => 0x29,
            TableKind::Param => 0x08,
            TableKind::TypeDef => 0x02,
            TableKind::TypeRef => 0x01,
        }
    }
    
}