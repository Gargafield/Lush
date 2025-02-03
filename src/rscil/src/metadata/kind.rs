
use super::*;

macro_rules! table_kind_impl {
    {#[$($attr:meta)*]
    $visibility:vis enum $name:ident {
        $(
            $(#[$($field_attr:meta)*])*
            $field:ident = $value:expr,
        )*
    }} => {
        #[$($attr)*]
        #[repr(u8)]
        $visibility enum $name {
            $(
                $(#[$($field_attr)*])*
                $field = $value,
            )*
        }

        impl From<u8> for $name {
            fn from(value: u8) -> Self {
                match value {
                    $(
                        $value => $name::$field,
                    )*
                    _ => panic!("Invalid table kind: {}", value),
                }
            }
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> u8 {
                match value {
                    $(
                        $name::$field => $value,
                    )*
                }
            }
        }

    };
}

table_kind_impl!{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TableKind {
        /// # II.22.2 Assembly : 0x20
        /// See [`Assembly`]
        Assembly = 0x20,
        /// # II.22.3 AssemblyOS : 0x22
        /// [...]
        /// 
        /// It shall be ignored by the CLI. 
        AssemblyOS = 0x22,
        /// # II.22.4 AssemblyProcessor : 0x21
        /// [...]
        /// It should be ignored by the CLI.
        AssemblyProcessor = 0x21,
        /// # II.22.5 AssemblyRef : 0x23
        /// See [`AssemblyRef`]
        AssemblyRef = 0x23,
        /// # II.22.6 AssemblyRefOS : 0x25
        /// [...]
        /// They should be ignored by the CLI.
        AssemblyRefOS = 0x25,
        /// # II.22.7 AssemblyRefProcessor : 0x24
        /// [...]
        /// They should be ignored by the CLI.
        AssemblyRefProcessor = 0x24,
        /// # II.22.8 ClassLayout : 0x0F
        /// See [`ClassLayout`]
        ClassLayout = 0x0F,
        /// II.22.9 Constant : 0x0B
        /// See [`Constant`]
        Constant = 0x0B,
        /// # II.22.10 CustomAttribute : 0x0C
        CustomAttribute = 0x0C,
        /// # II.22.11 DeclSecurity : 0x0E
        /// See [`DeclSecurity`]
        DeclSecurity = 0x0E,
        /// # II.22.12 EventMap : 0x12
        /// See [`EventMap`]
        EventMap = 0x12,
        /// # II.22.13 Event : 0x14
        /// See [`Event`]
        Event = 0x14,
        /// # II.22.14 ExportedType : 0x27
        /// See [`ExportedType`]
        ExportedType = 0x27,
        /// # II.22.15 Field : 0x04
        /// See [`Field`]
        Field = 0x04,
        /// # II.22.16 FieldLayout : 0x10
        /// See [`FieldLayout`]
        FieldLayout = 0x10,
        /// # II.22.17 FieldMarshal : 0x0D
        /// See [`FieldMarshal`]
        FieldMarshal = 0x0D,
        /// # II.22.18 FieldRVA : 0x1D
        /// See [`FieldRVA`]
        FieldRVA = 0x1D,
        /// # II.22.19 File : 0x26
        /// See [`File`]
        File = 0x26,
        /// # II.22.20 GenericParam : 0x2A
        /// See [`GenericParam`]
        GenericParam = 0x2A,
        /// # II.22.21 GenericParamConstraint : 0x2C
        /// See [`GenericParamConstraint`]
        GenericParamConstraint = 0x2C,
        /// # II.22.22 ImplMap : 0x1C
        /// See [`ImplMap`]
        ImplMap = 0x1C,
        /// # II.22.23 InterfaceImpl : 0x09
        /// See [`InterfaceImpl`]
        InterfaceImpl = 0x09,
        /// # II.22.24 ManifestResource : 0x28
        /// See [`ManifestResource`]
        ManifestResource = 0x28,
        /// # II.22.25 MemberRef : 0x0A 
        /// See [`MemberRef`]
        MemberRef = 0x0A,
        /// # II.22.26 MethodDef : 0x06
        /// See [`MethodDef`]
        MethodDef = 0x06,
        /// # II.22.27 MethodImpl : 0x19
        /// See [`MethodImpl`]
        MethodImpl = 0x19,
        /// # II.22.28 MethodSemantics : 0x18
        /// See [`MethodSemantics`]
        MethodSemantics = 0x18,
        /// # II.22.29 MethodSpec : 0x2B
        /// See [`MethodSpec`]
        MethodSpec = 0x2B,
        /// # II.22.30 Module : 0x00 
        /// See [`Module`]
        Module = 0x00,
        /// # II.22.31 ModuleRef : 0x1A
        /// See [`ModuleRef`]
        ModuleRef = 0x1A,
        /// # II.22.32 NestedClass : 0x29
        /// See [`NestedClass`]
        NestedClass = 0x29,
        /// II.22.33 Param : 0x08
        /// See [`Param`]
        Param = 0x08,
        /// # II.22.34 Property : 0x17
        /// See [`Property`]
        Property = 0x17,
        /// # II.22.35 PropertyMap : 0x15
        /// See [`PropertyMap`]
        PropertyMap = 0x15,
        /// # II.22.36 StandAloneSig : 0x11
        /// See [`StandAloneSig`]
        StandAloneSig = 0x11,
        /// # II.22.37 TypeDef : 0x02
        /// See [`TypeDef`]
        TypeDef = 0x02,
        /// # II.22.38 TypeRef : 0x01
        /// See [`TypeRef`]
        TypeRef = 0x01,
        /// # II.22.39 TypeSpec : 0x1B
        /// See [`TypeSpec`]
        TypeSpec = 0x1B,
    }    
}

impl TableKind {
    pub const NUM_TABLES: usize = 45;

    pub fn from_bitmask(bitmask: u64) -> Vec<TableKind> {
        let mut kinds = Vec::new();
        for i in 0..64 {
            if (bitmask & (1 << i)) != 0 {
                kinds.push(TableKind::from(i));
            }
        }
        kinds
    }
}

impl TableEnumDecode for TableKind {
    type Output = CodedIndex;

    fn decode(self, context: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self::Output, std::io::Error> {
        if context.get_table_index_size(self) == 2 {
            Ok(CodedIndex::from(self, buffer.read_u16::<LittleEndian>()? as u32))
        }
        else {
            Ok(CodedIndex::from(self, buffer.read_u32::<LittleEndian>()?))
        }
    }
}