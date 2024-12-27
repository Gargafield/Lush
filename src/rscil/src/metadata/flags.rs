#![allow(dead_code)]

/// II.23.1.2 Values for AssemblyFlags
/// 
/// | Flag                       | Value    | Description |
/// | -------------------------- | -------- | ----------- |
/// | PublicKey                  | `0x0001` | The assembly reference holds the full (unhashed) public key. |
/// | Retargetable               | `0x0100` | The implementation of this assembly used at runtime is not expected to match the version seen at compile time. (See the text following this table.) |
/// | DisableJITcompileOptimizer | `0x4000` | Reserved (a conforming implementation of the CLI can ignore this setting on read; some implementations might use this bit to indicate that a CIL-to-native-code compiler should not generate optimized code) |
/// | EnableJITcompileTracking   | `0x8000` | Reserved  (a conforming implementation of the CLI can ignore this setting on read; some implementations might use this bit to indicate that a CIL-to-native-code compiler should generate CIL-to-native code map) |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssemblyFlags(u16);

impl From<u16> for AssemblyFlags {
    fn from(value: u16) -> Self {
        AssemblyFlags(value)
    }
}

impl AssemblyFlags {
    const PUBLIC_KEY: u16 = 0x0001;
    const RETARGETABLE: u16 = 0x0100;
    const DISABLE_JIT_COMPILE_OPTIMIZER: u16 = 0x4000;
    const ENABLE_JIT_COMPILE_TRACKING: u16 = 0x8000;
    
    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// II.23.1.4 Flags for events [EventAttributes] 
///
/// | Flag            | Value    | Description | 
/// | --------------- | -------- | ----------- |
/// | `SpecialName`   | `0x0200` | Event is special. |
/// | `RTSpecialName` | `0x0400` | CLI provides 'special' behavior, depending upon the name of the event |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventAttributes(u16);

impl From<u16> for EventAttributes {
    fn from(value: u16) -> Self {
        EventAttributes(value)
    }
}

impl EventAttributes {
    const SPECIAL_NAME: u16 = 0x0200;
    const RT_SPECIAL_NAME: u16 = 0x0400;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// II.23.1.5 Flags for fields [FieldAttributes]
/// 
/// | Flag                    | Value    | Description                                                                 |
/// | ----------------------- | -------- | --------------------------------------------------------------------------- |
/// | `FieldAccessMask`       | `0x0007` | These 3 bits contain one of the following values:                           |
/// | - `CompilerControlled`  | `0x0000` | Member not referenceable                                                    |
/// | - `Private`             | `0x0001` | Accessible only by the parent type                                          |
/// | - `FamANDAssem`         | `0x0002` | Accessible by sub-types only in this Assembly                               |
/// | - `Assembly`            | `0x0003` | Accessible by anyone in the Assembly                                        |
/// | - `Family`              | `0x0004` | Accessible only by type and sub-types                                       |
/// | - `FamORAssem`          | `0x0005` | Accessible by sub-types anywhere, plus anyone in assembly                   |
/// | - `Public`              | `0x0006` | Accessible by anyone who has visibility to this scope field                 |
/// | `Static`                | `0x0010` | Defined on type, else per instance                                          |
/// | `InitOnly`              | `0x0020` | Field can only be initialized, not written to after init                    |
/// | `Literal`               | `0x0040` | Value is compile time constant                                              |
/// | `NotSerialized`         | `0x0080` | Reserved (to indicate this field should not be serialized when type is remoted) |
/// | `SpecialName`           | `0x0200` | Field is special                                                            |
/// | Interop Attributes      |          | |
/// | `PInvokeImpl`           | `0x2000` | Implementation is forwarded through PInvoke.                                |
/// | Additional flags        |          | |
/// | `RTSpecialName`         | `0x0400` | CLI provides 'special' behavior, depending upon the name of the field       |
/// | `HasFieldMarshal`       | `0x1000` | Field has marshalling information                                           |
/// | `HasDefault`            | `0x8000` | Field has default                                                           |
/// | `HasFieldRVA`           | `0x0100` | Field has RVA                                                               |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldAttributes (u16);

impl From<u16> for FieldAttributes {
    fn from(value: u16) -> Self {
        FieldAttributes(value)
    }
}

impl FieldAttributes {
    const FIELD_ACCESS_MASK: u16 = 0x0007;
    const COMPILER_CONTROLLED: u16 = 0x0000;
    const PRIVATE: u16 = 0x0001;
    const FAM_AND_ASSEM: u16 = 0x0002;
    const ASSEMBLY: u16 = 0x0003;
    const FAMILY: u16 = 0x0004;
    const FAM_OR_ASSEM: u16 = 0x0005;
    const PUBLIC: u16 = 0x0006;
    const STATIC: u16 = 0x0010;
    const INIT_ONLY: u16 = 0x0020;
    const LITERAL: u16 = 0x0040;
    const NOT_SERIALIZED: u16 = 0x0080;
    const SPECIAL_NAME: u16 = 0x0200;
    const PINVOKE_IMPL: u16 = 0x2000;
    const RT_SPECIAL_NAME: u16 = 0x0400;
    const HAS_FIELD_MARSHAL: u16 = 0x1000;
    const HAS_DEFAULT: u16 = 0x8000;
    const HAS_FIELD_RVA: u16 = 0x0100;
}

/// II.23.1.6 Flags for files [FileAttributes]
/// 
/// | Flag                 | Value    | Description  |
/// | -------------------- | -------- | ------------ |
/// | `ContainsMetaData`   | `0x0000` | This is not a resource file  |
/// | `ContainsNoMetaData` | `0x0001` | This is a resource file or other non-metadata-containing file |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileAttributes(u32);

impl From<u32> for FileAttributes {
    fn from(value: u32) -> Self {
        FileAttributes(value)
    }
}

impl FileAttributes {
    const CONTAINS_META_DATA: u32 = 0x0000;
    const CONTAINS_NO_META_DATA: u32 = 0x0001;

    pub fn check_flag(&self, flag: u32) -> bool {
        self.0 & flag == flag
    }
}

/// II.23.1.7 Flags for Generic Parameters [GenericParamAttributes] 
/// 
/// | Flag                               | Value    | Description |
/// | ---------------------------------- | -------- | ----------- |
/// | `VarianceMask`                     | `0x0003` | These 2 bits contain one of the following values: |
/// | - `None`                           | `0x0000` | The generic parameter is non-variant and has no special constraints |
/// | - `Covariant`                      | `0x0001` | The generic parameter is covariant |
/// | - `Contravariant`                  | `0x0002` | The generic parameter is contravariant |
/// | `SpecialConstraintMask`            | `0x001C` | These 3 bits contain one of the following values: |
/// | - `ReferenceTypeConstraint`        | `0x0004` | The generic parameter has the class special constraint |
/// | - `NotNullableValueTypeConstraint` | `0x0008` | The generic parameter has the valuetype special constraint |
/// | - `DefaultConstructorConstraint`   | `0x0010` | The generic parameter has the .ctor special constraint|
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericParamAttributes(u16);

impl From<u16> for GenericParamAttributes {
    fn from(value: u16) -> Self {
        GenericParamAttributes(value)
    }
}

impl GenericParamAttributes {
    const VARIANCE_MASK: u16 = 0x0003;
    const NONE: u16 = 0x0000;
    const COVARIANT: u16 = 0x0001;
    const CONTRAVARIANT: u16 = 0x0002;
    const SPECIAL_CONSTRAINT_MASK: u16 = 0x001C;
    const REFERENCE_TYPE_CONSTRAINT: u16 = 0x0004;
    const NOT_NULLABLE_VALUE_TYPE_CONSTRAINT: u16 = 0x0008;
    const DEFAULT_CONSTRUCTOR_CONSTRAINT: u16 = 0x0010;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// II.23.1.8 Flags for ImplMap [PInvokeAttributes] 
/// 
/// | Flag                    | Value    | Description |
/// | ----------------------- | -------- | ----------- |
/// | `NoMangle`              | `0x0001` | PInvoke is to use the member name as specified |
/// | Character set           |          | | 
/// | `CharSetMask`           | `0x0006` | This is a resource file or other non-metadata-containing file. These 2 bits contain one of the following values: |
/// | - `CharSetNotSpec`      | `0x0000` | |
/// | - `CharSetAnsi`         | `0x0002` | |
/// | - `CharSetUnicode`      | `0x0004` | |
/// | - `CharSetAuto`         | `0x0006` | |
/// | `SupportsLastError`     | `0x0040` | Information about target function. Not relevant for fields |
/// | Calling convention      |          | | 
/// | `CallConvMask`          | `0x0700` | These 3 bits contain one of the following values: |
/// | - `CallConvPlatformapi` | `0x0100` | |
/// | - `CallConvCdecl`       | `0x0200` | |
/// | - `CallConvStdcall`     | `0x0300` | |
/// | - `CallConvThiscall`    | `0x0400` | |
/// | - `CallConvFastcall`    | `0x0500` | |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PInvokeAttributes(u16);

impl From<u16> for PInvokeAttributes {
    fn from(value: u16) -> Self {
        PInvokeAttributes(value)
    }
}

impl PInvokeAttributes {
    const NO_MANGLE: u16 = 0x0001;
    const CHAR_SET_MASK: u16 = 0x0006;
    const CHAR_SET_NOT_SPEC: u16 = 0x0000;
    const CHAR_SET_ANSI: u16 = 0x0002;
    const CHAR_SET_UNICODE: u16 = 0x0004;
    const CHAR_SET_AUTO: u16 = 0x0006;
    const SUPPORTS_LAST_ERROR: u16 = 0x0040;
    const CALL_CONV_MASK: u16 = 0x0700;
    const CALL_CONV_PLATFORM_API: u16 = 0x0100;
    const CALL_CONV_CDECL: u16 = 0x0200;
    const CALL_CONV_STDCALL: u16 = 0x0300;
    const CALL_CONV_THISCALL: u16 = 0x0400;
    const CALL_CONV_FASTCALL: u16 = 0x0500;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// II.23.1.9 Flags for ManifestResource [ManifestResourceAttributes] 
///
/// | Flag             | Value    | Description |
/// | ---------------- | -------- | ----------- |
/// | `VisibilityMask` | `0x0007` | These 3 bits contain one of the following values: |
/// | `Public`         | `0x0001` | The Resource is exported from the Assembly |
/// | `Private`        | `0x0002` | The Resource is private to the Assembly |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestResourceAttributes(u32);

impl From<u32> for ManifestResourceAttributes {
    fn from(value: u32) -> Self {
        ManifestResourceAttributes(value)
    }
}

impl ManifestResourceAttributes {
    const VISIBILITY_MASK: u32 = 0x0007;
    const PUBLIC: u32 = 0x0001;
    const PRIVATE: u32 = 0x0002;

    pub fn check_flag(&self, flag: u32) -> bool {
        self.0 & flag == flag
    }
}

/// II.23.1.10 Flags for methods [MethodAttributes]
/// 
/// | Flag                   | Value    | Description |
/// | ---------------------- | -------- | ----------- |
/// | `MemberAccessMask`     | `0x0007` | These 3 bits contain one of the following values: |
/// | - `CompilerControlled` | `0x0000` | Member not referenceable |
/// | - `Private`            | `0x0001` | Accessible only by the parent type |
/// | - `FamANDAssem`        | `0x0002` | Accessible by sub-types only in this Assembly |
/// | - `Assem`              | `0x0003` | Accessibly by anyone in the Assembly |
/// | - `Family`             | `0x0004` | Accessible only by type and sub-types |
/// | - `FamORAssem`         | `0x0005` | Accessibly by sub-types anywhere, plus anyone in assembly |
/// | - `Public`             | `0x0006` | Accessibly by anyone who has visibility to this scope |
/// | `Static`               | `0x0010` | Defined on type, else per instance |
/// | `Final`                | `0x0020` | Method cannot be overridden |
/// | `Virtual`              | `0x0040` | Method is virtual |
/// | `HideBySig`            | `0x0080` | Method hides by name+sig, else just by name |
/// | `VtableLayoutMask`     | `0x0100` | Use this mask to retrieve vtable attributes. This bit contains one of the following values: |
/// | - `ReuseSlot`          | `0x0000` | Method reuses existing slot in vtable |
/// | - `NewSlot`            | `0x0100` | Method always gets a new slot in the vtable |
/// | `Strict`               | `0x0200` | Method can only be overriden if also accessible |
/// | `Abstract`             | `0x0400` | Method does not provide an implementation |
/// | `SpecialName`          | `0x0800` | Method is special |
/// | Interop attributes     |          | |
/// | `PInvokeImpl`          | `0x2000` | Implementation is forwarded through PInvoke |
/// | `UnmanagedExport`      | `0x0008` | Reserved: shall be zero for conforming implementations |
/// | Additional  flags      |          | |
/// | `RTSpecialName`        | `0x1000` | CLI provides 'special' behavior, depending upon the name of the method |
/// | `HasSecurity`          | `0x4000` | Method has security associate with it |
/// | `RequireSecObject`     | `0x8000` | Method calls another method containing security code. |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodAttributes(u16);

impl From<u16> for MethodAttributes {
    fn from(value: u16) -> Self {
        MethodAttributes(value)
    }
}

impl MethodAttributes {
    const MEMBER_ACCESS_MASK: u16 = 0x0007;
    const COMPILER_CONTROLLED: u16 = 0x0000;
    const PRIVATE: u16 = 0x0001;
    const FAM_AND_ASSEM: u16 = 0x0002;
    const ASSEM: u16 = 0x0003;
    const FAMILY: u16 = 0x0004;
    const FAM_OR_ASSEM: u16 = 0x0005;
    const PUBLIC: u16 = 0x0006;
    const STATIC: u16 = 0x0010;
    const FINAL: u16 = 0x0020;
    const VIRTUAL: u16 = 0x0040;
    const HIDE_BY_SIG: u16 = 0x0080;
    const VTABLE_LAYOUT_MASK: u16 = 0x0100;
    const REUSE_SLOT: u16 = 0x0000;
    const NEW_SLOT: u16 = 0x0100;
    const STRICT: u16 = 0x0200;
    const ABSTRACT: u16 = 0x0400;
    const SPECIAL_NAME: u16 = 0x0800;
    const PINVOKE_IMPL: u16 = 0x2000;
    const UNMANAGED_EXPORT: u16 = 0x0008;
    const RT_SPECIAL_NAME: u16 = 0x1000;
    const HAS_SECURITY: u16 = 0x4000;
    const REQUIRE_SEC_OBJECT: u16 = 0x8000;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// # II.23.1.11 Flags for methods [MethodImplAttributes] 
/// | Flag                            | Value    | Description |
/// | ------------------------------- | -------- | ----------- |
/// | `CodeTypeMask`                  | `0x0003` | These 2 bits contain one of the following values: |
/// | - `IL`                          | `0x0000` | Method impl is CIL |
/// | - `Native`                      | `0x0001` | Method impl is native |
/// | - `OPTIL`                       | `0x0002` | Reserved: shall be zero in conforming implementations |
/// | - `Runtime`                     | `0x0003` | Method impl is provided by the runtime |
/// | `ManagedMask`                   | `0x0004` | Flags specifying whether the code is managed or unmanaged. This bit contains one of the following values: |
/// | - `Unmanaged`                   | `0x0004` | Method impl is unmanaged, otherwise managed |
/// | - `Managed`                     | `0x0000` | Method impl is managed |
/// | Implementation info and interop |          | |
/// | `ForwardRef`                    | `0x0010` | Indicates method is defined; used primarily in merge scenarios |
/// | `PreserveSig`                   | `0x0080` | Reserved: conforming implementations can ignore |
/// | `InternalCall`                  | `0x1000` | Reserved: shall be zero in conforming implementations |
/// | `Synchronized`                  | `0x0020` | Method is single threaded through the body |
/// | `NoInlining`                    | `0x0008` | Method cannot be inlined |
/// | `MaxMethodImplVal`              | `0xffff` | Range check value |
/// | `NoOptimization`                | `0x0040` | Method will not be optimized when generating native code |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodImplAttributes(u16);

impl From<u16> for MethodImplAttributes {
    fn from(value: u16) -> Self {
        MethodImplAttributes(value)
    }
}

impl MethodImplAttributes {
    const CODE_TYPE_MASK: u16 = 0x0003;
    const IL: u16 = 0x0000;
    const NATIVE: u16 = 0x0001;
    const OPTIL: u16 = 0x0002;
    const RUNTIME: u16 = 0x0003;
    const MANAGED_MASK: u16 = 0x0004;
    const UNMANAGED: u16 = 0x0004;
    const MANAGED: u16 = 0x0000;
    const FORWARD_REF: u16 = 0x0010;
    const PRESERVE_SIG: u16 = 0x0080;
    const INTERNAL_CALL: u16 = 0x1000;
    const SYNCHRONIZED: u16 = 0x0020;
    const NO_INLINING: u16 = 0x0008;
    const MAX_METHOD_IMPL_VAL: u16 = 0xffff;
    const NO_OPTIMIZATION: u16 = 0x0040;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// # II.23.1.12 Flags for MethodSemantics [MethodSemanticsAttributes] 
/// 
/// | Flag       | Value    | Description |
/// | ---------- | -------- | ----------- |
/// | `Setter`   | `0x0001` | Setter for property |
/// | `Getter`   | `0x0002` | Getter for property |
/// | `Other`    | `0x0004` | Other method for property or event |
/// | `AddOn`    | `0x0008` | AddOn method for event. This refers to the required `add_` method for events.  (§22.13) |
/// | `RemoveOn` | `0x0010` | RemoveOn method for event. . This refers to the required `remove_` method for events. (§22.13) |
/// | `Fire`     | `0x0020` | Fire method for event. This refers to the optional `raise_` method for events. (§22.13)|
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodSemanticsAttributes(u16);

impl From<u16> for MethodSemanticsAttributes {
    fn from(value: u16) -> Self {
        MethodSemanticsAttributes(value)
    }
}

impl MethodSemanticsAttributes {
    const SETTER: u16 = 0x0001;
    const GETTER: u16 = 0x0002;
    const OTHER: u16 = 0x0004;
    const ADD_ON: u16 = 0x0008;
    const REMOVE_ON: u16 = 0x0010;
    const FIRE: u16 = 0x0020;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// # II.23.1.13 Flags for params [ParamAttributes] 
/// | Flag              | Value    | Description |
/// | ----------------- | -------- | ----------- |
/// | `In`              | `0x0001` | Param is [In] |
/// | `Out`             | `0x0002` | Param is [out] |
/// | `Optional`        | `0x0010` | Param is optional |
/// | `HasDefault`      | `0x1000` | Param has default value |
/// | `HasFieldMarshal` | `0x2000` | Param has FieldMarshal |
/// | `Unused`          | `0xcfe0` | Reserved: shall be zero in a conforming implementation |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamAttributes(u16);

impl From<u16> for ParamAttributes {
    fn from(value: u16) -> Self {
        ParamAttributes(value)
    }
}

impl ParamAttributes {
    const IN: u16 = 0x0001;
    const OUT: u16 = 0x0002;
    const OPTIONAL: u16 = 0x0010;
    const HAS_DEFAULT: u16 = 0x1000;
    const HAS_FIELD_MARSHAL: u16 = 0x2000;
    const UNUSED: u16 = 0xcfe0;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// # II.23.1.14 Flags for properties [PropertyAttributes] 
/// 
/// | Flag            | Value    | Description |
/// | --------------- | -------- | ----------- |
/// | `SpecialName`   | `0x0200` | Property is special |
/// | `RTSpecialName` | `0x0400` | Runtime(metadata internal APIs) should check name encoding |
/// | `HasDefault`    | `0x1000` | Property has default |
/// | `Unused`        | `0xe9ff` | Reserved: shall be zero in a conforming implementation |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyAttributes(u16);

impl From<u16> for PropertyAttributes {
    fn from(value: u16) -> Self {
        PropertyAttributes(value)
    }
}

impl PropertyAttributes {
    const SPECIAL_NAME: u16 = 0x0200;
    const RT_SPECIAL_NAME: u16 = 0x0400;
    const HAS_DEFAULT: u16 = 0x1000;
    const UNUSED: u16 = 0xe9ff;

    pub fn check_flag(&self, flag: u16) -> bool {
        self.0 & flag == flag
    }
}

/// # II.23.1.15 Flags for types [TypeAttributes] 
/// 
/// | Flag                                             | Value        | Description |
/// | ------------------------------------------------ | ------------ | ----------- |
/// | Visibility attributes                            |              | |
/// | `VisibilityMask`                                 | `0x00000007` | Use this mask to retrieve visibility information. These 3 bits contain one of the following values: |
/// | - `NotPublic`                                    | `0x00000000` | Class has no public scope |
/// | - `Public`                                       | `0x00000001` | Class has public scope |
/// | - `NestedPublic`                                 | `0x00000002` | Class is nested with public visibility |
/// | - `NestedPrivate`                                | `0x00000003` | Class is nested with private visibility |
/// | - `NestedFamily`                                 | `0x00000004` | Class is nested with family visibility |
/// | - `NestedAssembly`                               | `0x00000005` | Class is nested with assembly visibility |
/// | - `NestedFamANDAssem`                            | `0x00000006` | Class is nested with family and assembly visibility |
/// | - `NestedFamORAssem`                             | `0x00000007` | Class is nested with family or assembly visibility |
/// | Class layout attributes                          |              | |
/// | `LayoutMask`                                     | `0x00000018` | Use this mask to retrieve class layout information. These 2 bits contain one of the following values: |
/// | - `AutoLayout`                                   | `0x00000000` | Class fields are auto-laid out |
/// | - `SequentialLayout`                             | `0x00000008` | Class fields are laid out sequentially |
/// | - `ExplicitLayout`                               | `0x00000010` | Layout is supplied explicitly |
/// | Class semantics attributes                       |              | |
/// | `ClassSemanticsMask`                             | `0x00000020` | Use this mask to retrive class semantics information. This bit contains one of the following values: |
/// | - `Class`                                        | `0x00000000` | Type is a class |
/// | - `Interface`                                    | `0x00000020` | Type is an interface |
/// | Special semantics in addition to class semantics |              | |
/// | `Abstract`                                       | `0x00000080` | Class is abstract |
/// | `Sealed`                                         | `0x00000100` | Class cannot be extended |
/// | `SpecialName`                                    | `0x00000400` | Class name is special |
/// | Implementation Attributes                        |              | |
/// | `Import`                                         | `0x00001000` | Class/Interface is imported |
/// | `Serializable`                                   | `0x00002000` | Reserved (Class is serializable) |
/// | String formatting Attributes                     |              | |
/// | `StringFormatMask`                               | `0x00030000` | Use this mask to retrieve string information for native interop. These 2 bits contain one of the following values: |
/// | - `AnsiClass`                                    | `0x00000000` | LPSTR is interpreted as ANSI |
/// | - `UnicodeClass`                                 | `0x00010000` | LPSTR is interpreted as Unicode |
/// | - `AutoClass`                                    | `0x00020000` | LPSTR is interpreted automatically |
/// | - `CustomFormatClass`                            | `0x00030000` | A non-standard encoding specified by `CustomStringFormatMask` |
/// | `CustomStringFormatMask`                         | `0x00C00000` | Use this mask to retrieve non-standard encoding information for native interop. The meaning of the values of these 2 bits is unspecified. |
/// | Class Initialization Attributes                  |              | |
/// | `BeforeFieldInit`                                | `0x00100000` | Initialize the class before first static field access |
/// | Additional Flags                                 |              | |
/// | `RTSpecialName`                                  | `0x00000800` | CLI provides 'special' behavior, depending upon the name of the Type |
/// | `HasSecurity`                                    | `0x00040000` | Type has security associate with it |
/// | `IsTypeForwarder`                                | `0x00200000` | This ExportedType entry is a type forwarder |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeAttributes(u32);

impl From<u32> for TypeAttributes {
    fn from(value: u32) -> Self {
        TypeAttributes(value)
    }
}

impl TypeAttributes {
    const VISIBILITY_MASK: u32 = 0x00000007;
    const NOT_PUBLIC: u32 = 0x00000000;
    const PUBLIC: u32 = 0x00000001;
    const NESTED_PUBLIC: u32 = 0x00000002;
    const NESTED_PRIVATE: u32 = 0x00000003;
    const NESTED_FAMILY: u32 = 0x00000004;
    const NESTED_ASSEMBLY: u32 = 0x00000005;
    const NESTED_FAM_AND_ASSEM: u32 = 0x00000006;
    const NESTED_FAM_OR_ASSEM: u32 = 0x00000007;
    const LAYOUT_MASK: u32 = 0x00000018;
    const AUTO_LAYOUT: u32 = 0x00000000;
    const SEQUENTIAL_LAYOUT: u32 = 0x00000008;
    const EXPLICIT_LAYOUT: u32 = 0x00000010;
    const CLASS_SEMANTICS_MASK: u32 = 0x00000020;
    const CLASS: u32 = 0x00000000;
    const INTERFACE: u32 = 0x00000020;
    const ABSTRACT: u32 = 0x00000080;
    const SEALED: u32 = 0x00000100;
    const SPECIAL_NAME: u32 = 0x00000400;
    const IMPORT: u32 = 0x00001000;
    const SERIALIZABLE: u32 = 0x00002000;
    const STRING_FORMAT_MASK: u32 = 0x00030000;
    const ANSI_CLASS: u32 = 0x00000000;
    const UNICODE_CLASS: u32 = 0x00010000;
    const AUTO_CLASS: u32 = 0x00020000;
    const CUSTOM_FORMAT_CLASS: u32 = 0x00030000;
    const CUSTOM_STRING_FORMAT_MASK: u32 = 0x00C00000;
    const BEFORE_FIELD_INIT: u32 = 0x00100000;
    const RT_SPECIAL_NAME: u32 = 0x00000800;
    const HAS_SECURITY: u32 = 0x00040000;
    const IS_TYPE_FORWARDER: u32 = 0x00200000;

    pub fn check_flag(&self, flag: u32) -> bool {
        self.0 & flag == flag
    }
}