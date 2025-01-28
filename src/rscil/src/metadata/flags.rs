#![allow(dead_code)]

use super::*;
use paste::paste;

macro_rules! flags {
    ($name:ident { $($flag:ident = $value:expr),* $(,)? }) => {
        flags!($name : u16 { $($flag = $value),* });
    };

    ($name:ident : $size:ident { $($flag:ident = $value:expr),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name($size);

        impl $name {
            $(
                pub const $flag: Self = Self($value);
            )*

            pub fn new(value: $size) -> Self {
                Self(value)
            }

            pub fn contains(&self, flag: Self) -> bool {
                self.0 & flag.0 == flag.0
            }

            pub fn check_flag(&self, flag: $size) -> bool {
                self.0 & flag == flag
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl From<$size> for $name {
            fn from(value: $size) -> Self {
                $name(value)
            }
        }

        impl From<$name> for $size {
            fn from(value: $name) -> $size {
                value.0
            }
        }

        impl TableDecode for $name {
            type Output = Self;
        
            fn decode(_: &TableDecodeContext, buffer: &mut Buffer) -> Result<Self, std::io::Error> {
                paste! {
                    let value = flags!(@read buffer $size);
                    Ok(Self::new(value))
                }
            }
        }
    };
    (@read $buffer:ident u8) => { $buffer.read_u8()? };
    (@read $buffer:ident $ty:ty) => { paste! { $buffer.[<read_ $ty:lower>]::<LittleEndian>()? } };
}

// II.23.1.2 Values for AssemblyFlags
// 
// | Flag                       | Value    | Description |
// | -------------------------- | -------- | ----------- |
// | PublicKey                  | `0x0001` | The assembly reference holds the full (unhashed) public key. |
// | Retargetable               | `0x0100` | The implementation of this assembly used at runtime is not expected to match the version seen at compile time. (See the text following this table.) |
// | DisableJITcompileOptimizer | `0x4000` | Reserved (a conforming implementation of the CLI can ignore this setting on read; some implementations might use this bit to indicate that a CIL-to-native-code compiler should not generate optimized code) |
// | EnableJITcompileTracking   | `0x8000` | Reserved  (a conforming implementation of the CLI can ignore this setting on read; some implementations might use this bit to indicate that a CIL-to-native-code compiler should generate CIL-to-native code map) |
flags!(AssemblyFlags {
    PUBLIC_KEY = 0x0001,
    RETARGETABLE = 0x0100,
    DISABLE_JIT_COMPILE_OPTIMIZER = 0x4000,
    ENABLE_JIT_COMPILE_TRACKING = 0x8000,
});

// II.23.1.4 Flags for events [EventAttributes] 
//
// | Flag            | Value    | Description | 
// | --------------- | -------- | ----------- |
// | `SpecialName`   | `0x0200` | Event is special. |
// | `RTSpecialName` | `0x0400` | CLI provides 'special' behavior, depending upon the name of the event |
flags!(EventAttributes {
    SPECIAL_NAME = 0x0200,
    RT_SPECIAL_NAME = 0x0400,
});

// II.23.1.5 Flags for fields [FieldAttributes]
// 
// | Flag                    | Value    | Description                                                                 |
// | ----------------------- | -------- | --------------------------------------------------------------------------- |
// | `FieldAccessMask`       | `0x0007` | These 3 bits contain one of the following values:                           |
// | - `CompilerControlled`  | `0x0000` | Member not referenceable                                                    |
// | - `Private`             | `0x0001` | Accessible only by the parent type                                          |
// | - `FamANDAssem`         | `0x0002` | Accessible by sub-types only in this Assembly                               |
// | - `Assembly`            | `0x0003` | Accessible by anyone in the Assembly                                        |
// | - `Family`              | `0x0004` | Accessible only by type and sub-types                                       |
// | - `FamORAssem`          | `0x0005` | Accessible by sub-types anywhere, plus anyone in assembly                   |
// | - `Public`              | `0x0006` | Accessible by anyone who has visibility to this scope field                 |
// | `Static`                | `0x0010` | Defined on type, else per instance                                          |
// | `InitOnly`              | `0x0020` | Field can only be initialized, not written to after init                    |
// | `Literal`               | `0x0040` | Value is compile time constant                                              |
// | `NotSerialized`         | `0x0080` | Reserved (to indicate this field should not be serialized when type is remoted) |
// | `SpecialName`           | `0x0200` | Field is special                                                            |
// | Interop Attributes      |          | |
// | `PInvokeImpl`           | `0x2000` | Implementation is forwarded through PInvoke.                                |
// | Additional flags        |          | |
// | `RTSpecialName`         | `0x0400` | CLI provides 'special' behavior, depending upon the name of the field       |
// | `HasFieldMarshal`       | `0x1000` | Field has marshalling information                                           |
// | `HasDefault`            | `0x8000` | Field has default                                                           |
// | `HasFieldRVA`           | `0x0100` | Field has RVA                                                               |
flags!(FieldAttributes {
    FIELD_ACCESS_MASK = 0x0007,
    COMPILER_CONTROLLED = 0x0000,
    PRIVATE = 0x0001,
    FAM_AND_ASSEM = 0x0002,
    ASSEMBLY = 0x0003,
    FAMILY = 0x0004,
    FAM_OR_ASSEM = 0x0005,
    PUBLIC = 0x0006,
    STATIC = 0x0010,
    INIT_ONLY = 0x0020,
    LITERAL = 0x0040,
    NOT_SERIALIZED = 0x0080,
    SPECIAL_NAME = 0x0200,
    PINVOKE_IMPL = 0x2000,
    RT_SPECIAL_NAME = 0x0400,
    HAS_FIELD_MARSHAL = 0x1000,
    HAS_DEFAULT = 0x8000,
    HAS_FIELD_RVA = 0x0100,
});

// II.23.1.6 Flags for files [FileAttributes]
// 
// | Flag                 | Value    | Description  |
// | -------------------- | -------- | ------------ |
// | `ContainsMetaData`   | `0x0000` | This is not a resource file  |
// | `ContainsNoMetaData` | `0x0001` | This is a resource file or other non-metadata-containing file |
flags!(FileAttributes : u32 {
    CONTAINS_META_DATA = 0x0000,
    CONTAINS_NO_META_DATA = 0x0001,
});

// II.23.1.7 Flags for Generic Parameters [GenericParamAttributes] 
// 
// | Flag                               | Value    | Description |
// | ---------------------------------- | -------- | ----------- |
// | `VarianceMask`                     | `0x0003` | These 2 bits contain one of the following values: |
// | - `None`                           | `0x0000` | The generic parameter is non-variant and has no special constraints |
// | - `Covariant`                      | `0x0001` | The generic parameter is covariant |
// | - `Contravariant`                  | `0x0002` | The generic parameter is contravariant |
// | `SpecialConstraintMask`            | `0x001C` | These 3 bits contain one of the following values: |
// | - `ReferenceTypeConstraint`        | `0x0004` | The generic parameter has the class special constraint |
// | - `NotNullableValueTypeConstraint` | `0x0008` | The generic parameter has the valuetype special constraint |
// | - `DefaultConstructorConstraint`   | `0x0010` | The generic parameter has the .ctor special constraint|
flags!(GenericParamAttributes {
    VARIANCE_MASK = 0x0003,
    NONE = 0x0000,
    COVARIANT = 0x0001,
    CONTRAVARIANT = 0x0002,
    SPECIAL_CONSTRAINT_MASK = 0x001C,
    REFERENCE_TYPE_CONSTRAINT = 0x0004,
    NOT_NULLABLE_VALUE_TYPE_CONSTRAINT = 0x0008,
    DEFAULT_CONSTRUCTOR_CONSTRAINT = 0x0010,
});

// II.23.1.8 Flags for ImplMap [PInvokeAttributes] 
// 
// | Flag                    | Value    | Description |
// | ----------------------- | -------- | ----------- |
// | `NoMangle`              | `0x0001` | PInvoke is to use the member name as specified |
// | Character set           |          | | 
// | `CharSetMask`           | `0x0006` | This is a resource file or other non-metadata-containing file. These 2 bits contain one of the following values: |
// | - `CharSetNotSpec`      | `0x0000` | |
// | - `CharSetAnsi`         | `0x0002` | |
// | - `CharSetUnicode`      | `0x0004` | |
// | - `CharSetAuto`         | `0x0006` | |
// | `SupportsLastError`     | `0x0040` | Information about target function. Not relevant for fields |
// | Calling convention      |          | | 
// | `CallConvMask`          | `0x0700` | These 3 bits contain one of the following values: |
// | - `CallConvPlatformapi` | `0x0100` | |
// | - `CallConvCdecl`       | `0x0200` | |
// | - `CallConvStdcall`     | `0x0300` | |
// | - `CallConvThiscall`    | `0x0400` | |
// | - `CallConvFastcall`    | `0x0500` | |
flags!(PInvokeAttributes {
    NO_MANGLE = 0x0001,
    CHAR_SET_MASK = 0x0006,
    CHAR_SET_NOT_SPEC = 0x0000,
    CHAR_SET_ANSI = 0x0002,
    CHAR_SET_UNICODE = 0x0004,
    CHAR_SET_AUTO = 0x0006,
    SUPPORTS_LAST_ERROR = 0x0040,
    CALL_CONV_MASK = 0x0700,
    CALL_CONV_PLATFORM_API = 0x0100,
    CALL_CONV_CDECL = 0x0200,
    CALL_CONV_STDCALL = 0x0300,
    CALL_CONV_THISCALL = 0x0400,
    CALL_CONV_FASTCALL = 0x0500,
});

// II.23.1.9 Flags for ManifestResource [ManifestResourceAttributes] 
//
// | Flag             | Value    | Description |
// | ---------------- | -------- | ----------- |
// | `VisibilityMask` | `0x0007` | These 3 bits contain one of the following values: |
// | `Public`         | `0x0001` | The Resource is exported from the Assembly |
// | `Private`        | `0x0002` | The Resource is private to the Assembly |
flags!(ManifestResourceAttributes : u32 {
    VISIBILITY_MASK = 0x0007,
    PUBLIC = 0x0001,
    PRIVATE = 0x0002,
});

// II.23.1.10 Flags for methods [MethodAttributes]
// 
// | Flag                   | Value    | Description |
// | ---------------------- | -------- | ----------- |
// | `MemberAccessMask`     | `0x0007` | These 3 bits contain one of the following values: |
// | - `CompilerControlled` | `0x0000` | Member not referenceable |
// | - `Private`            | `0x0001` | Accessible only by the parent type |
// | - `FamANDAssem`        | `0x0002` | Accessible by sub-types only in this Assembly |
// | - `Assem`              | `0x0003` | Accessibly by anyone in the Assembly |
// | - `Family`             | `0x0004` | Accessible only by type and sub-types |
// | - `FamORAssem`         | `0x0005` | Accessibly by sub-types anywhere, plus anyone in assembly |
// | - `Public`             | `0x0006` | Accessibly by anyone who has visibility to this scope |
// | `Static`               | `0x0010` | Defined on type, else per instance |
// | `Final`                | `0x0020` | Method cannot be overridden |
// | `Virtual`              | `0x0040` | Method is virtual |
// | `HideBySig`            | `0x0080` | Method hides by name+sig, else just by name |
// | `VtableLayoutMask`     | `0x0100` | Use this mask to retrieve vtable attributes. This bit contains one of the following values: |
// | - `ReuseSlot`          | `0x0000` | Method reuses existing slot in vtable |
// | - `NewSlot`            | `0x0100` | Method always gets a new slot in the vtable |
// | `Strict`               | `0x0200` | Method can only be overriden if also accessible |
// | `Abstract`             | `0x0400` | Method does not provide an implementation |
// | `SpecialName`          | `0x0800` | Method is special |
// | Interop attributes     |          | |
// | `PInvokeImpl`          | `0x2000` | Implementation is forwarded through PInvoke |
// | `UnmanagedExport`      | `0x0008` | Reserved: shall be zero for conforming implementations |
// | Additional  flags      |          | |
// | `RTSpecialName`        | `0x1000` | CLI provides 'special' behavior, depending upon the name of the method |
// | `HasSecurity`          | `0x4000` | Method has security associate with it |
// | `RequireSecObject`     | `0x8000` | Method calls another method containing security code. |
flags!(MethodAttributes {
    MEMBER_ACCESS_MASK = 0x0007,
    COMPILER_CONTROLLED = 0x0000,
    PRIVATE = 0x0001,
    FAM_AND_ASSEM = 0x0002,
    ASSEM = 0x0003,
    FAMILY = 0x0004,
    FAM_OR_ASSEM = 0x0005,
    PUBLIC = 0x0006,
    STATIC = 0x0010,
    FINAL = 0x0020,
    VIRTUAL = 0x0040,
    HIDE_BY_SIG = 0x0080,
    VTABLE_LAYOUT_MASK = 0x0100,
    REUSE_SLOT = 0x0000,
    NEW_SLOT = 0x0100,
    STRICT = 0x0200,
    ABSTRACT = 0x0400,
    SPECIAL_NAME = 0x0800,
    PINVOKE_IMPL = 0x2000,
    UNMANAGED_EXPORT = 0x0008,
    RT_SPECIAL_NAME = 0x1000,
    HAS_SECURITY = 0x4000,
    REQUIRE_SEC_OBJECT = 0x8000,
});

// # II.23.1.11 Flags for methods [MethodImplAttributes] 
// | Flag                            | Value    | Description |
// | ------------------------------- | -------- | ----------- |
// | `CodeTypeMask`                  | `0x0003` | These 2 bits contain one of the following values: |
// | - `IL`                          | `0x0000` | Method impl is CIL |
// | - `Native`                      | `0x0001` | Method impl is native |
// | - `OPTIL`                       | `0x0002` | Reserved: shall be zero in conforming implementations |
// | - `Runtime`                     | `0x0003` | Method impl is provided by the runtime |
// | `ManagedMask`                   | `0x0004` | Flags specifying whether the code is managed or unmanaged. This bit contains one of the following values: |
// | - `Unmanaged`                   | `0x0004` | Method impl is unmanaged, otherwise managed |
// | - `Managed`                     | `0x0000` | Method impl is managed |
// | Implementation info and interop |          | |
// | `ForwardRef`                    | `0x0010` | Indicates method is defined; used primarily in merge scenarios |
// | `PreserveSig`                   | `0x0080` | Reserved: conforming implementations can ignore |
// | `InternalCall`                  | `0x1000` | Reserved: shall be zero in conforming implementations |
// | `Synchronized`                  | `0x0020` | Method is single threaded through the body |
// | `NoInlining`                    | `0x0008` | Method cannot be inlined |
// | `MaxMethodImplVal`              | `0xffff` | Range check value |
// | `NoOptimization`                | `0x0040` | Method will not be optimized when generating native code |
flags!(MethodImplAttributes {
    CODE_TYPE_MASK = 0x0003,
    IL = 0x0000,
    NATIVE = 0x0001,
    OPTIL = 0x0002,
    RUNTIME = 0x0003,
    MANAGED_MASK = 0x0004,
    UNMANAGED = 0x0004,
    MANAGED = 0x0000,
    FORWARD_REF = 0x0010,
    PRESERVE_SIG = 0x0080,
    INTERNAL_CALL = 0x1000,
    SYNCHRONIZED = 0x0020,
    NO_INLINING = 0x0008,
    MAX_METHOD_IMPL_VAL = 0xffff,
    NO_OPTIMIZATION = 0x0040,
});

// # II.23.1.12 Flags for MethodSemantics [MethodSemanticsAttributes] 
// 
// | Flag       | Value    | Description |
// | ---------- | -------- | ----------- |
// | `Setter`   | `0x0001` | Setter for property |
// | `Getter`   | `0x0002` | Getter for property |
// | `Other`    | `0x0004` | Other method for property or event |
// | `AddOn`    | `0x0008` | AddOn method for event. This refers to the required `add_` method for events.  (§22.13) |
// | `RemoveOn` | `0x0010` | RemoveOn method for event. . This refers to the required `remove_` method for events. (§22.13) |
// | `Fire`     | `0x0020` | Fire method for event. This refers to the optional `raise_` method for events. (§22.13)|
flags!(MethodSemanticsAttributes {
    SETTER = 0x0001,
    GETTER = 0x0002,
    OTHER = 0x0004,
    ADD_ON = 0x0008,
    REMOVE_ON = 0x0010,
    FIRE = 0x0020,
});

// # II.23.1.13 Flags for params [ParamAttributes] 
// | Flag              | Value    | Description |
// | ----------------- | -------- | ----------- |
// | `In`              | `0x0001` | Param is [In] |
// | `Out`             | `0x0002` | Param is [out] |
// | `Optional`        | `0x0010` | Param is optional |
// | `HasDefault`      | `0x1000` | Param has default value |
// | `HasFieldMarshal` | `0x2000` | Param has FieldMarshal |
// | `Unused`          | `0xcfe0` | Reserved: shall be zero in a conforming implementation |
flags!(ParamAttributes {
    IN = 0x0001,
    OUT = 0x0002,
    OPTIONAL = 0x0010,
    HAS_DEFAULT = 0x1000,
    HAS_FIELD_MARSHAL = 0x2000,
    UNUSED = 0xcfe0,
});

// # II.23.1.14 Flags for properties [PropertyAttributes] 
// 
// | Flag            | Value    | Description |
// | --------------- | -------- | ----------- |
// | `SpecialName`   | `0x0200` | Property is special |
// | `RTSpecialName` | `0x0400` | Runtime(metadata internal APIs) should check name encoding |
// | `HasDefault`    | `0x1000` | Property has default |
// | `Unused`        | `0xe9ff` | Reserved: shall be zero in a conforming implementation |
flags!(PropertyAttributes {
    SPECIAL_NAME = 0x0200,
    RT_SPECIAL_NAME = 0x0400,
    HAS_DEFAULT = 0x1000,
    UNUSED = 0xe9ff,
});

// # II.23.1.15 Flags for types [TypeAttributes] 
// 
// | Flag                                             | Value        | Description |
// | ------------------------------------------------ | ------------ | ----------- |
// | Visibility attributes                            |              | |
// | `VisibilityMask`                                 | `0x00000007` | Use this mask to retrieve visibility information. These 3 bits contain one of the following values: |
// | - `NotPublic`                                    | `0x00000000` | Class has no public scope |
// | - `Public`                                       | `0x00000001` | Class has public scope |
// | - `NestedPublic`                                 | `0x00000002` | Class is nested with public visibility |
// | - `NestedPrivate`                                | `0x00000003` | Class is nested with private visibility |
// | - `NestedFamily`                                 | `0x00000004` | Class is nested with family visibility |
// | - `NestedAssembly`                               | `0x00000005` | Class is nested with assembly visibility |
// | - `NestedFamANDAssem`                            | `0x00000006` | Class is nested with family and assembly visibility |
// | - `NestedFamORAssem`                             | `0x00000007` | Class is nested with family or assembly visibility |
// | Class layout attributes                          |              | |
// | `LayoutMask`                                     | `0x00000018` | Use this mask to retrieve class layout information. These 2 bits contain one of the following values: |
// | - `AutoLayout`                                   | `0x00000000` | Class fields are auto-laid out |
// | - `SequentialLayout`                             | `0x00000008` | Class fields are laid out sequentially |
// | - `ExplicitLayout`                               | `0x00000010` | Layout is supplied explicitly |
// | Class semantics attributes                       |              | |
// | `ClassSemanticsMask`                             | `0x00000020` | Use this mask to retrive class semantics information. This bit contains one of the following values: |
// | - `Class`                                        | `0x00000000` | Type is a class |
// | - `Interface`                                    | `0x00000020` | Type is an interface |
// | Special semantics in addition to class semantics |              | |
// | `Abstract`                                       | `0x00000080` | Class is abstract |
// | `Sealed`                                         | `0x00000100` | Class cannot be extended |
// | `SpecialName`                                    | `0x00000400` | Class name is special |
// | Implementation Attributes                        |              | |
// | `Import`                                         | `0x00001000` | Class/Interface is imported |
// | `Serializable`                                   | `0x00002000` | Reserved (Class is serializable) |
// | String formatting Attributes                     |              | |
// | `StringFormatMask`                               | `0x00030000` | Use this mask to retrieve string information for native interop. These 2 bits contain one of the following values: |
// | - `AnsiClass`                                    | `0x00000000` | LPSTR is interpreted as ANSI |
// | - `UnicodeClass`                                 | `0x00010000` | LPSTR is interpreted as Unicode |
// | - `AutoClass`                                    | `0x00020000` | LPSTR is interpreted automatically |
// | - `CustomFormatClass`                            | `0x00030000` | A non-standard encoding specified by `CustomStringFormatMask` |
// | `CustomStringFormatMask`                         | `0x00C00000` | Use this mask to retrieve non-standard encoding information for native interop. The meaning of the values of these 2 bits is unspecified. |
// | Class Initialization Attributes                  |              | |
// | `BeforeFieldInit`                                | `0x00100000` | Initialize the class before first static field access |
// | Additional Flags                                 |              | |
// | `RTSpecialName`                                  | `0x00000800` | CLI provides 'special' behavior, depending upon the name of the Type |
// | `HasSecurity`                                    | `0x00040000` | Type has security associate with it |
// | `IsTypeForwarder`                                | `0x00200000` | This ExportedType entry is a type forwarder |
flags!(TypeAttributes : u32 {
    VISIBILITY_MASK = 0x00000007,
    NOT_PUBLIC = 0x00000000,
    PUBLIC = 0x00000001,
    NESTED_PUBLIC = 0x00000002,
    NESTED_PRIVATE = 0x00000003,
    NESTED_FAMILY = 0x00000004,
    NESTED_ASSEMBLY = 0x00000005,
    NESTED_FAM_AND_ASSEM = 0x00000006,
    NESTED_FAM_OR_ASSEM = 0x00000007,
    LAYOUT_MASK = 0x00000018,
    AUTO_LAYOUT = 0x00000000,
    SEQUENTIAL_LAYOUT = 0x00000008,
    EXPLICIT_LAYOUT = 0x00000010,
    CLASS_SEMANTICS_MASK = 0x00000020,
    CLASS = 0x00000000,
    INTERFACE = 0x00000020,
    ABSTRACT = 0x00000080,
    SEALED = 0x00000100,
    SPECIAL_NAME = 0x00000400,
    IMPORT = 0x00001000,
    SERIALIZABLE = 0x00002000,
    STRING_FORMAT_MASK = 0x00030000,
    ANSI_CLASS = 0x00000000,
    UNICODE_CLASS = 0x00010000,
    AUTO_CLASS = 0x00020000,
    CUSTOM_FORMAT_CLASS = 0x00030000,
    CUSTOM_STRING_FORMAT_MASK = 0x00C00000,
    BEFORE_FIELD_INIT = 0x00100000,
    RT_SPECIAL_NAME = 0x00000800,
    HAS_SECURITY = 0x00040000,
    IS_TYPE_FORWARDER = 0x00200000,
});

// # II.25.2.2.1 Characteristics
// 
// | Flag                           | Value    | Description |
// | ------------------------------ | -------- | ----------- |
// | `IMAGE_FILE_RELOCS_STRIPPED`   | `0x0001` | Shall be zero |
// | `IMAGE_FILE_EXECUTABLE_IMAGE`  | `0x0002` | Shall be one |
// | `IMAGE_FILE_32BIT_MACHINE`     | `0x0100` | Shall be one if and only if `COMIMAGE_FLAGS_32BITREQUIRED` is one (25.3.3.1) |
// | `IMAGE_FILE_DLL`               | `0x2000` | The image file is a dynamic-link library (DLL). |
// 
// For the flags not mentioned above, flags 0x0010, 0x0020, 0x0400 and 0x0800 are implementation specific, and all others should be zero (§II.24.1).
flags!(FileCharacteristics {
    IMAGE_FILE_RELOCS_STRIPPED = 0x0001,
    IMAGE_FILE_EXECUTABLE_IMAGE = 0x0002,
    IMAGE_FILE_32BIT_MACHINE = 0x0100,
    IMAGE_FILE_DLL = 0x2000,
});

// # II.25.3 Section headers 
// 
// [...]
// 
// The following table defines the possible characteristics of the section. 
// 
// | Flag                               | Value        | Description |
// | ---------------------------------- | ------------ | ----------- |
// | `IMAGE_SCN_CNT_CODE`               | `0x00000020` | Section contains code. |
// | `IMAGE_SCN_CNT_INITIALIZED_DATA`   | `0x00000040` | Section contains initialized data. |
// | `IMAGE_SCN_CNT_UNINITIALIZED_DATA` | `0x00000080` | Section contains uninitialized data. |
// | `IMAGE_SCN_MEM_EXECUTE`            | `0x20000000` | Section can be executed as code. |
// | `IMAGE_SCN_MEM_READ`               | `0x40000000` | Section can be read. |
// | `IMAGE_SCN_MEM_WRITE`              | `0x80000000` | Section can be written to.|
flags!(SectionCharacteristics : u32 {
    IMAGE_SCN_CNT_CODE = 0x00000020,
    IMAGE_SCN_CNT_INITIALIZED_DATA = 0x00000040,
    IMAGE_SCN_CNT_UNINITIALIZED_DATA = 0x00000080,
    IMAGE_SCN_MEM_EXECUTE = 0x20000000,
    IMAGE_SCN_MEM_READ = 0x40000000,
    IMAGE_SCN_MEM_WRITE = 0x80000000,
});

// # II.25.3.3.1 Runtime flags 
// 
// The following flags describe this runtime image and are used by the loader. All unspecified bits should 
// be zero.
// 
// | Flag                               | Value        | Description |
// | ---------------------------------- | ------------ | ----------- |
// | `COMIMAGE_FLAGS_ILONLY`            | `0x00000001` | Shall be 1. |
// | `COMIMAGE_FLAGS_32BITREQUIRED`     | `0x00000002` | Image can only be loaded into a 32-bit process, for instance if there are 32-bit vtablefixups, or casts from native integers to int32. CLI implementations that have 64-bit native integers shall refuse loading binaries with this flag set. |
// | `COMIMAGE_FLAGS_STRONGNAMESIGNED`  | `0x00000008` | Image has a strong name signature. |
// | `COMIMAGE_FLAGS_NATIVE_ENTRYPOINT` | `0x00000010` | Shall be 0. |
// | `COMIMAGE_FLAGS_TRACKDEBUGDATA`    | `0x00010000` | Should be 0 (§II.24.1). |
flags!(RuntimeFlags : u32 {
    COMIMAGE_FLAGS_ILONLY = 0x00000001,
    COMIMAGE_FLAGS_32BITREQUIRED = 0x00000002,
    COMIMAGE_FLAGS_STRONGNAMESIGNED = 0x00000008,
    COMIMAGE_FLAGS_NATIVE_ENTRYPOINT = 0x00000010,
    COMIMAGE_FLAGS_TRACKDEBUGDATA = 0x00010000,
});

// # II.25.4.1 Method header type values 
//
// The two least significant bits of the first byte of the method header indicate what type of header is 
// present. These 2 bits will be one and only one of the following:
// 
// | Value                    | Value | Description | 
// | ------------------------ | ----- | ----------- |
// | `CorILMethod_TinyFormat` | `0x2` | The method header is tiny (§II.25.4.2). | 
// | `CorILMethod_FatFormat`  | `0x3` | The method header is fat (§II.25.4.3). |
// 
// # II.25.4.4 Flags for method headers 
// 
// The first byte of a method header can also contain the following flags, valid only for the Fat format, 
// that indicate how the method is to be executed:
// 
// | Flag                     | Value  | Description |
// | ------------------------ | ------ | ----------- |
// | [...]                    | [...]  | [...] |
// | `CorILMethod_MoreSects`  | `0x8`  | More sections follow after this header (§II.25.4.5). |
// | `CorILMethod_InitLocals` | `0x10` | Call default constructor on all local variables. |
// 
flags!(MethodHeaderType : u8 {
    COR_IL_METHOD_TINY_FORMAT = 0x2,
    COR_IL_METHOD_FAT_FORMAT = 0x3,
    COR_IL_METHOD_MORE_SECTS = 0x8,
    COR_IL_METHOD_INIT_LOCALS = 0x10,
});

impl MethodHeaderType {
    pub fn is_tiny_format(&self) -> bool {
        self.contains(MethodHeaderType::COR_IL_METHOD_TINY_FORMAT)
    }

    pub fn is_fat_format(&self) -> bool {
        self.contains(MethodHeaderType::COR_IL_METHOD_FAT_FORMAT)
    }
}