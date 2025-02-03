use paste::paste;

use super::*;

#[derive(Debug)]
pub struct MethodBody {
    pub body: Vec<Instruction>,
    pub max_stack: u16,
    pub code_size: u32,
    // TODO: Exception handling
    // TODO: Local variables
}

impl MethodBody {
    /// # [II.25.4.2](https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=311) Tiny format 
    ///
    /// Tiny headers use a 6-bit length encoding. The following is true for all tiny headers: 
    /// * No local variables are allowed 
    /// * No exceptions 
    /// * No extra data sections 
    /// * The operand stack shall be no bigger than 8 entries 
    /// 
    /// A Tiny Format header is encoded as follows: 
    /// 
    /// | Start Bit | Count of Bits | Description |
    /// | --------- |- ------------ | ----------- |
    /// | 0         | 2             | Flags ([`MethodHeaderType::COR_IL_METHOD_TINY_FORMAT`] shall be set, see §II.25.4.4). |
    /// | 2         | 6             | Size, in bytes, of the method body immediately following this header. |
    pub fn tiny(byte: u8) -> MethodBody {
        let code_size = (byte >> 2) as u32;
        MethodBody {
            body: Vec::with_capacity(code_size as usize),
            max_stack: 8,
            code_size,
        }
    }

    /// # [II.25.4.3] Fat format 
    /// 
    /// [...]
    /// 
    /// A fat header has the following structure
    /// 
    /// | Offset    | Size      | **Field**          | Description |
    /// | --------- | --------- | ------------------ |-------------|
    /// | 0         | 12 (bits) | **Flags**          | Flags ([`MethodHeaderType::COR_IL_METHOD_FAT_FORMAT`] shall be set in bits 0:1, see §[II.25.4.4]) |
    /// | 12 (bits) | 4 (bits)  | **Size**           | Size of this header expressed as the count of 4-byte integers occupied (currently 3) |
    /// | 2         | 2         | **MaxStack**       | Maximum number of items on the operand stack |
    /// | 4         | 4         | **CodeSize**       | Size in bytes of the actual method body |
    /// | 8         | 4         | **LocalVarSigTok** | Meta Data token for a signature describing the layout of the local variables for the method. 0 means there are no local variables present |
    /// 
    /// [II.25.4.3]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=311
    /// [II.25.4.4]: https://www.ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf#page=311
    pub fn fat(bytes: &[u8]) -> MethodBody {
        let _flags: u16 = u16::from_le_bytes([bytes[0], bytes[1]]);
        let max_stack :u16 = u16::from_le_bytes([bytes[2], bytes[3]]);
        let code_size: u32 = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let _local_var_sig_tok: u32 = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        
        MethodBody {
            body: Vec::with_capacity(code_size as usize),
            max_stack,
            code_size,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub offset: u32,
}

#[derive(Debug)]
pub struct CodeMetadata {
    pub code: Code,
    pub name: &'static str,
    pub operand_type: OperandType,
    pub stack_behaviour_pop: StackBehaviour,
    pub stack_behaviour_push: StackBehaviour,
    pub flow_control: FlowControl,
}

impl CodeMetadata {
    const fn new(code: Code, name: &'static str, operand_type: OperandType, stack_behaviour_pop: StackBehaviour, stack_behaviour_push: StackBehaviour, flow_control: FlowControl) -> CodeMetadata {
        CodeMetadata {
            code,
            name,
            operand_type,
            stack_behaviour_pop,
            stack_behaviour_push,
            flow_control,
        }
    }
}

#[derive(Debug)]
pub enum OperandType {
    InlineBrTarget ,
    InlineField,
    InlineI,
    InlineI8,
    InlineMethod,
    InlineNone,
    InlineR,
    InlineSig,
    InlineString,
    InlineSwitch,
    InlineTok,
    InlineType,
    InlineVar,
    ShortInlineBrTarget,
    ShortInlineI,
    ShortInlineR,
    ShortInlineVar,
}

#[derive(Debug)]
pub enum StackBehaviour {
    Pop0,
    Pop1,
    Pop1Pop1,
    PopI,
    PopIPop1,
    PopIPopI,
    PopIPopIPopI,
    PopIPopI8,
    PopIPopR4,
    PopIPopR8,
    PopRef,
    PopRefPopI,
    PopRefPopIPopI,
    PopRefPopIPopI8,
    PopRefPopIPopR4,
    PopRefPopIPopR8,
    PopRefPopIPopRef,
    VarPop,
    Push0,
    Push1,
    Push1Push1,
    PushI,
    PushI8,
    PushR4,
    PushR8,
    PushRef,
    VarPush,
}

#[derive(Debug)]
pub enum FlowControl {
    Next,
    Call,
    Return,
    Branch,
    CondBranch,
    Switch,
    Throw,
    Break,
    Meta
}

/// # III.3.66 switch – table switch based on value 
/// 
/// [...]
/// 
/// The format of the instruction is an unsigned int32 representing the number of targets N,
/// followed by N int32 values specifying jump targets:
/// these targets are represented as offsets (positive or negative) from the beginning of the instruction following this switch instruction.
fn read_switch_table(buffer: &mut Buffer) -> Result<Vec<i32>, std::io::Error> {
    let count = buffer.read_u32::<LittleEndian>()? as usize;
    let mut table = Vec::with_capacity(count);
    for _ in 0..count {
        table.push(buffer.read_i32::<LittleEndian>()?);
    }
    Ok(table)
}

macro_rules! opcodes {
    ($(OPDEF($name:ident, $instr:tt, $pop:ident, $push:ident, $operand:ident, $_type:ident, $size:tt, $op1:tt, $op2:tt, $flow:ident))*) => {
        paste! {
            opcodes!(@def $(
                [<$name:camel>], $instr, $pop, $push, $operand, $op1, $op2, $flow
            )*);
        }
    };

    (@def $($name:ident, $instr:tt, $pop:ident, $push:ident, $operand:ident, $op1:tt, $op2:tt, $flow:ident)*) => {
        paste! {
            #[derive(Debug)]
            pub enum Code {
                $($name,)*
            }

            impl Code {
                $(
                    const [<$name:upper _METADATA>]: CodeMetadata = CodeMetadata::new(Code::$name, $instr, OperandType::$operand, StackBehaviour::$pop, StackBehaviour::$push, FlowControl::$flow);
                )*
            
                pub fn metadata(&self) -> CodeMetadata {
                    match self {
                        $(
                            Code::$name => Self::[<$name:upper _METADATA>],
                        )*
                    }
                }

                pub fn from(slice: &[u8]) -> Code {
                    match slice {
                        $(
                            [$op2, $op1, ..] => Code::$name,
                        )*
                        _ => panic!("Invalid opcode: {:?}", slice),
                    }
                }
            }

            #[derive(Debug)]
            pub enum OpCode {
                $($name ( opcodes!(@ty $operand) ),)*
            }

            impl OpCode {
                pub fn code(&self) -> Code {
                    match self {
                        $(
                            OpCode::$name (_) => Code::$name,
                        )*
                    }
                }

                pub fn parse(code: Code, buffer: &mut Buffer) -> Result<OpCode, std::io::Error> {
                    match code {
                        $(
                            Code::$name => Ok(OpCode::$name( opcodes!(@parse $operand buffer) )),
                        )*
                    }
                }
            }
        }
    };

    // # VI.C.2 CIL opcode descriptions
    // Type of in-line argument to instruction. The in-line argument is stored with least significant byte first (“little endian”).
    // The possible values here are the following:
    //
    // a. InlineBrTarget – Branch target, represented as a 4-byte signed integer from the beginning of the instruction following the current instruction. 
    (@ty InlineBrTarget) => { i32 };
    (@parse InlineBrTarget $buffer:ident) => { $buffer.read_i32::<LittleEndian>()? };

    // b. InlineField – Metadata token (4 bytes) representing a FieldRef (i.e., a MemberRef to a field) or FieldDef 
    (@ty InlineField) => { MetadataToken };
    (@parse InlineField $buffer:ident) => { MetadataToken::read($buffer)? };

    // c. InlineI – 4-byte integer 
    (@ty InlineI) => { i32 };
    (@parse InlineI $buffer:ident) => { $buffer.read_i32::<LittleEndian>()? };

    // d. InlineI8 – 8-byte integer 
    (@ty InlineI8) => { i64 };
    (@parse InlineI8 $buffer:ident) => { $buffer.read_i64::<LittleEndian>()? };

    // e. InlineMethod – Metadata token (4 bytes) representing a MethodRef (i.e., a MemberRef to a method) or MethodDef 
    (@ty InlineMethod) => { MetadataToken };
    (@parse InlineMethod $buffer:ident) => { MetadataToken::read($buffer)? };

    // f. InlineNone – No in-line argument 
    (@ty InlineNone) => { () };
    (@parse InlineNone $buffer:ident) => { () };
    
    // g. InlineR – 8-byte floating point number 
    (@ty InlineR) => { f64 };
    (@parse InlineR $buffer:ident) => { $buffer.read_f64::<LittleEndian>()? };

    // h. InlineSig – Metadata token (4 bytes) representing a standalone signature 
    (@ty InlineSig) => { MetadataToken };
    (@parse InlineSig $buffer:ident) => { MetadataToken::read($buffer)? };

    // i. InlineString – Metadata token (4 bytes) representing a UserString 
    (@ty InlineString) => { MetadataToken };
    (@parse InlineString $buffer:ident) => { MetadataToken::read($buffer)? };

    // j. InlineSwitch – Special for the switch instructions, see [`PeParser::read_switch_table`] for details
    (@ty InlineSwitch) => { Vec<i32> };
    (@parse InlineSwitch $buffer:ident) => { read_switch_table($buffer)? };

    // k. InlineTok – Arbitrary metadata token (4 bytes) , used for ldtoken instruction, see Partition III for details 
    (@ty InlineTok) => { MetadataToken };
    (@parse InlineTok $buffer:ident) => { MetadataToken::read($buffer)? };

    // l. InlineType – Metadata token (4 bytes) representing a TypeDef, TypeRef, or TypeSpec
    (@ty InlineType) => { MetadataToken };
    (@parse InlineType $buffer:ident) => { MetadataToken::read($buffer)? };

    // m. InlineVar – 2-byte integer representing an argument or local variable
    (@ty InlineVar) => { u16 };
    (@parse InlineVar $buffer:ident) => { $buffer.read_u16::<LittleEndian>()? };

    // n. ShortInlineBrTarget – Short branch target, represented as 1 signed byte from the beginning of the instruction following the current instruction.
    (@ty ShortInlineBrTarget) => { i8 }; 
    (@parse ShortInlineBrTarget $buffer:ident) => { $buffer.read_i8()? };

    // o. ShortInlineI – 1-byte integer, signed or unsigned depending on instruction 
    (@ty ShortInlineI) => { i8 };
    (@parse ShortInlineI $buffer:ident) => { $buffer.read_i8()? };

    // p. ShortInlineR – 4-byte floating point number 
    (@ty ShortInlineR) => { f32 };
    (@parse ShortInlineR $buffer:ident) => { $buffer.read_f32::<LittleEndian>()? };
    
    // q. ShortInlineVar – 1-byte integer representing an argument or local variable 
    (@ty ShortInlineVar) => { u8 };
    (@parse ShortInlineVar $buffer:ident) => { $buffer.read_u8()? };
}

opcodes!(
    OPDEF(NOP,"nop",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0x00,Next)
    OPDEF(Break,"break",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0x01,Break)
    OPDEF(LDARG_0,"ldarg.0",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x02,Next)
    OPDEF(LDARG_1,"ldarg.1",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x03,Next)
    OPDEF(LDARG_2,"ldarg.2",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x04,Next)
    OPDEF(LDARG_3,"ldarg.3",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x05,Next)
    OPDEF(LDLOC_0,"ldloc.0",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x06,Next)
    OPDEF(LDLOC_1,"ldloc.1",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x07,Next)
    OPDEF(LDLOC_2,"ldloc.2",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x08,Next)
    OPDEF(LDLOC_3,"ldloc.3",Pop0,Push1,InlineNone,IMacro,1,0xFF,0x09,Next)
    OPDEF(STLOC_0,"stloc.0",Pop1,Push0,InlineNone,IMacro,1,0xFF,0x0A,Next)
    OPDEF(STLOC_1,"stloc.1",Pop1,Push0,InlineNone,IMacro,1,0xFF,0x0B,Next)
    OPDEF(STLOC_2,"stloc.2",Pop1,Push0,InlineNone,IMacro,1,0xFF,0x0C,Next)
    OPDEF(STLOC_3,"stloc.3",Pop1,Push0,InlineNone,IMacro,1,0xFF,0x0D,Next)
    OPDEF(LDARG_S,"ldarg.s",Pop0,Push1,ShortInlineVar,IMacro,1,0xFF,0x0E,Next)
    OPDEF(LDARGA_S,"ldarga.s",Pop0,PushI,ShortInlineVar,IMacro,1,0xFF,0x0F,Next)
    OPDEF(STARG_S,"starg.s",Pop1,Push0,ShortInlineVar,IMacro,1,0xFF,0x10,Next)
    OPDEF(LDLOC_S,"ldloc.s",Pop0,Push1,ShortInlineVar,IMacro,1,0xFF,0x11,Next)
    OPDEF(LDLOCA_S,"ldloca.s",Pop0,PushI,ShortInlineVar,IMacro,1,0xFF,0x12,Next)
    OPDEF(STLOC_S,"stloc.s",Pop1,Push0,ShortInlineVar,IMacro,1,0xFF,0x13,Next)
    OPDEF(LDNULL,"ldnull",Pop0,PushRef,InlineNone,IPrimitive,1,0xFF,0x14,Next)
    OPDEF(LDC_I4_M1,"ldc.i4.m1",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x15,Next)
    OPDEF(LDC_I4_0,"ldc.i4.0",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x16,Next)
    OPDEF(LDC_I4_1,"ldc.i4.1",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x17,Next)
    OPDEF(LDC_I4_2,"ldc.i4.2",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x18,Next)
    OPDEF(LDC_I4_3,"ldc.i4.3",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x19,Next)
    OPDEF(LDC_I4_4,"ldc.i4.4",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x1A,Next)
    OPDEF(LDC_I4_5,"ldc.i4.5",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x1B,Next)
    OPDEF(LDC_I4_6,"ldc.i4.6",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x1C,Next)
    OPDEF(LDC_I4_7,"ldc.i4.7",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x1D,Next)
    OPDEF(LDC_I4_8,"ldc.i4.8",Pop0,PushI,InlineNone,IMacro,1,0xFF,0x1E,Next)
    OPDEF(LDC_I4_S,"ldc.i4.s",Pop0,PushI,ShortInlineI,IMacro,1,0xFF,0x1F,Next)
    OPDEF(LDC_I4,"ldc.i4",Pop0,PushI,InlineI,IPrimitive,1,0xFF,0x20,Next)
    OPDEF(LDC_I8,"ldc.i8",Pop0,PushI8,InlineI8,IPrimitive,1,0xFF,0x21,Next)
    OPDEF(LDC_R4,"ldc.r4",Pop0,PushR4,ShortInlineR,IPrimitive,1,0xFF,0x22,Next)
    OPDEF(LDC_R8,"ldc.r8",Pop0,PushR8,InlineR,IPrimitive,1,0xFF,0x23,Next)
    OPDEF(UNUSED49,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0x24,Next)
    OPDEF(DUP,"dup",Pop1,Push1Push1,InlineNone,IPrimitive,1,0xFF,0x25,Next)
    OPDEF(POP,"pop",Pop1,Push0,InlineNone,IPrimitive,1,0xFF,0x26,Next)
    OPDEF(JMP,"jmp",Pop0,Push0,InlineMethod,IPrimitive,1,0xFF,0x27,Call)
    OPDEF(Call,"call",VarPop,VarPush,InlineMethod,IPrimitive,1,0xFF,0x28,Call)
    OPDEF(CALLI,"calli",VarPop,VarPush,InlineSig,IPrimitive,1,0xFF,0x29,Call)
    OPDEF(RET,"ret",VarPop,Push0,InlineNone,IPrimitive,1,0xFF,0x2A,Return)
    OPDEF(BR_S,"br.s",Pop0,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x2B,Branch)
    OPDEF(BRFALSE_S,"brfalse.s",PopI,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x2C,CondBranch)
    OPDEF(BRTRUE_S,"brtrue.s",PopI,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x2D,CondBranch)
    OPDEF(BEQ_S,"beq.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x2E,CondBranch)
    OPDEF(BGE_S,"bge.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x2F,CondBranch)
    OPDEF(BGT_S,"bgt.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x30,CondBranch)
    OPDEF(BLE_S,"ble.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x31,CondBranch)
    OPDEF(BLT_S,"blt.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x32,CondBranch)
    OPDEF(BNE_UN_S,"bne.un.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x33,CondBranch)
    OPDEF(BGE_UN_S,"bge.un.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x34,CondBranch)
    OPDEF(BGT_UN_S,"bgt.un.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x35,CondBranch)
    OPDEF(BLE_UN_S,"ble.un.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x36,CondBranch)
    OPDEF(BLT_UN_S,"blt.un.s",Pop1Pop1,Push0,ShortInlineBrTarget,IMacro,1,0xFF,0x37,CondBranch)
    OPDEF(BR,"br",Pop0,Push0,InlineBrTarget,IPrimitive,1,0xFF,0x38,Branch)
    OPDEF(BRFALSE,"brfalse",PopI,Push0,InlineBrTarget,IPrimitive,1,0xFF,0x39,CondBranch)
    OPDEF(BRTRUE,"brtrue",PopI,Push0,InlineBrTarget,IPrimitive,1,0xFF,0x3A,CondBranch)
    OPDEF(BEQ,"beq",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x3B,CondBranch)
    OPDEF(BGE,"bge",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x3C,CondBranch)
    OPDEF(BGT,"bgt",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x3D,CondBranch)
    OPDEF(BLE,"ble",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x3E,CondBranch)
    OPDEF(BLT,"blt",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x3F,CondBranch)
    OPDEF(BNE_UN,"bne.un",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x40,CondBranch)
    OPDEF(BGE_UN,"bge.un",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x41,CondBranch)
    OPDEF(BGT_UN,"bgt.un",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x42,CondBranch)
    OPDEF(BLE_UN,"ble.un",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x43,CondBranch)
    OPDEF(BLT_UN,"blt.un",Pop1Pop1,Push0,InlineBrTarget,IMacro,1,0xFF,0x44,CondBranch)
    OPDEF(Switch,"switch",PopI,Push0,InlineSwitch,IPrimitive,1,0xFF,0x45,CondBranch)
    OPDEF(LDIND_I1,"ldind.i1",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x46,Next)
    OPDEF(LDIND_U1,"ldind.u1",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x47,Next)
    OPDEF(LDIND_I2,"ldind.i2",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x48,Next)
    OPDEF(LDIND_U2,"ldind.u2",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x49,Next)
    OPDEF(LDIND_I4,"ldind.i4",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x4A,Next)
    OPDEF(LDIND_U4,"ldind.u4",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x4B,Next)
    OPDEF(LDIND_I8,"ldind.i8",PopI,PushI8,InlineNone,IPrimitive,1,0xFF,0x4C,Next)
    OPDEF(LDIND_I,"ldind.i",PopI,PushI,InlineNone,IPrimitive,1,0xFF,0x4D,Next)
    OPDEF(LDIND_R4,"ldind.r4",PopI,PushR4,InlineNone,IPrimitive,1,0xFF,0x4E,Next)
    OPDEF(LDIND_R8,"ldind.r8",PopI,PushR8,InlineNone,IPrimitive,1,0xFF,0x4F,Next)
    OPDEF(LDIND_REF,"ldind.ref",PopI,PushRef,InlineNone,IPrimitive,1,0xFF,0x50,Next)
    OPDEF(STIND_REF,"stind.ref",PopIPopI,Push0,InlineNone,IPrimitive,1,0xFF,0x51,Next)
    OPDEF(STIND_I1,"stind.i1",PopIPopI,Push0,InlineNone,IPrimitive,1,0xFF,0x52,Next)
    OPDEF(STIND_I2,"stind.i2",PopIPopI,Push0,InlineNone,IPrimitive,1,0xFF,0x53,Next)
    OPDEF(STIND_I4,"stind.i4",PopIPopI,Push0,InlineNone,IPrimitive,1,0xFF,0x54,Next)
    OPDEF(STIND_I8,"stind.i8",PopIPopI8,Push0,InlineNone,IPrimitive,1,0xFF,0x55,Next)
    OPDEF(STIND_R4,"stind.r4",PopIPopR4,Push0,InlineNone,IPrimitive,1,0xFF,0x56,Next)
    OPDEF(STIND_R8,"stind.r8",PopIPopR8,Push0,InlineNone,IPrimitive,1,0xFF,0x57,Next)
    OPDEF(ADD,"add",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x58,Next)
    OPDEF(SUB,"sub",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x59,Next)
    OPDEF(MUL,"mul",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x5A,Next)
    OPDEF(DIV,"div",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x5B,Next)
    OPDEF(DIV_UN,"div.un",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x5C,Next)
    OPDEF(REM,"rem",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x5D,Next)
    OPDEF(REM_UN,"rem.un",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x5E,Next)
    OPDEF(AND,"and",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x5F,Next)
    OPDEF(OR,"or",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x60,Next)
    OPDEF(XOR,"xor",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x61,Next)
    OPDEF(SHL,"shl",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x62,Next)
    OPDEF(SHR,"shr",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x63,Next)
    OPDEF(SHR_UN,"shr.un",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x64,Next)
    OPDEF(NEG,"neg",Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x65,Next)
    OPDEF(NOT,"not",Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0x66,Next)
    OPDEF(CONV_I1,"conv.i1",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x67,Next)
    OPDEF(CONV_I2,"conv.i2",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x68,Next)
    OPDEF(CONV_I4,"conv.i4",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x69,Next)
    OPDEF(CONV_I8,"conv.i8",Pop1,PushI8,InlineNone,IPrimitive,1,0xFF,0x6A,Next)
    OPDEF(CONV_R4,"conv.r4",Pop1,PushR4,InlineNone,IPrimitive,1,0xFF,0x6B,Next)
    OPDEF(CONV_R8,"conv.r8",Pop1,PushR8,InlineNone,IPrimitive,1,0xFF,0x6C,Next)
    OPDEF(CONV_U4,"conv.u4",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x6D,Next)
    OPDEF(CONV_U8,"conv.u8",Pop1,PushI8,InlineNone,IPrimitive,1,0xFF,0x6E,Next)
    OPDEF(CALLVIRT,"callvirt",VarPop,VarPush,InlineMethod,IObjModel,1,0xFF,0x6F,Call)
    OPDEF(CPOBJ,"cpobj",PopIPopI,Push0,InlineType,IObjModel,1,0xFF,0x70,Next)
    OPDEF(LDOBJ,"ldobj",PopI,Push1,InlineType,IObjModel,1,0xFF,0x71,Next)
    OPDEF(LDSTR,"ldstr",Pop0,PushRef,InlineString,IObjModel,1,0xFF,0x72,Next)
    OPDEF(NEWOBJ,"newobj",VarPop,PushRef,InlineMethod,IObjModel,1,0xFF,0x73,Call)
    OPDEF(CASTCLASS,"castclass",PopRef,PushRef,InlineType,IObjModel,1,0xFF,0x74,Next)
    OPDEF(ISINST,"isinst",PopRef,PushI,InlineType,IObjModel,1,0xFF,0x75,Next)
    OPDEF(CONV_R_UN,"conv.r.un",Pop1,PushR8,InlineNone,IPrimitive,1,0xFF,0x76,Next)
    OPDEF(UNUSED58,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0x77,Next)
    OPDEF(UNUSED1,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0x78,Next)
    OPDEF(UNBOX,"unbox",PopRef,PushI,InlineType,IPrimitive,1,0xFF,0x79,Next)
    OPDEF(Throw,"throw",PopRef,Push0,InlineNone,IObjModel,1,0xFF,0x7A,Throw)
    OPDEF(LDFLD,"ldfld",PopRef,Push1,InlineField,IObjModel,1,0xFF,0x7B,Next)
    OPDEF(LDFLDA,"ldflda",PopRef,PushI,InlineField,IObjModel,1,0xFF,0x7C,Next)
    OPDEF(STFLD,"stfld",PopRefPopI,Push0,InlineField,IObjModel,1,0xFF,0x7D,Next)
    OPDEF(LDSFLD,"ldsfld",Pop0,Push1,InlineField,IObjModel,1,0xFF,0x7E,Next)
    OPDEF(LDSFLDA,"ldsflda",Pop0,PushI,InlineField,IObjModel,1,0xFF,0x7F,Next)
    OPDEF(STSFLD,"stsfld",Pop1,Push0,InlineField,IObjModel,1,0xFF,0x80,Next)
    OPDEF(STOBJ,"stobj",PopIPop1,Push0,InlineType,IPrimitive,1,0xFF,0x81,Next)
    OPDEF(CONV_OVF_I1_UN,"conv.ovf.i1.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x82,Next)
    OPDEF(CONV_OVF_I2_UN,"conv.ovf.i2.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x83,Next)
    OPDEF(CONV_OVF_I4_UN,"conv.ovf.i4.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x84,Next)
    OPDEF(CONV_OVF_I8_UN,"conv.ovf.i8.un",Pop1,PushI8,InlineNone,IPrimitive,1,0xFF,0x85,Next)
    OPDEF(CONV_OVF_U1_UN,"conv.ovf.u1.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x86,Next)
    OPDEF(CONV_OVF_U2_UN,"conv.ovf.u2.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x87,Next)
    OPDEF(CONV_OVF_U4_UN,"conv.ovf.u4.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x88,Next)
    OPDEF(CONV_OVF_U8_UN,"conv.ovf.u8.un",Pop1,PushI8,InlineNone,IPrimitive,1,0xFF,0x89,Next)
    OPDEF(CONV_OVF_I_UN,"conv.ovf.i.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x8A,Next)
    OPDEF(CONV_OVF_U_UN,"conv.ovf.u.un",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0x8B,Next)
    OPDEF(BOX,"box",Pop1,PushRef,InlineType,IPrimitive,1,0xFF,0x8C,Next)
    OPDEF(NEWARR,"newarr",PopI,PushRef,InlineType,IObjModel,1,0xFF,0x8D,Next)
    OPDEF(LDLEN,"ldlen",PopRef,PushI,InlineNone,IObjModel,1,0xFF,0x8E,Next)
    OPDEF(LDELEMA,"ldelema",PopRefPopI,PushI,InlineType,IObjModel,1,0xFF,0x8F,Next)
    OPDEF(LDELEM_I1,"ldelem.i1",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x90,Next)
    OPDEF(LDELEM_U1,"ldelem.u1",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x91,Next)
    OPDEF(LDELEM_I2,"ldelem.i2",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x92,Next)
    OPDEF(LDELEM_U2,"ldelem.u2",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x93,Next)
    OPDEF(LDELEM_I4,"ldelem.i4",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x94,Next)
    OPDEF(LDELEM_U4,"ldelem.u4",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x95,Next)
    OPDEF(LDELEM_I8,"ldelem.i8",PopRefPopI,PushI8,InlineNone,IObjModel,1,0xFF,0x96,Next)
    OPDEF(LDELEM_I,"ldelem.i",PopRefPopI,PushI,InlineNone,IObjModel,1,0xFF,0x97,Next)
    OPDEF(LDELEM_R4,"ldelem.r4",PopRefPopI,PushR4,InlineNone,IObjModel,1,0xFF,0x98,Next)
    OPDEF(LDELEM_R8,"ldelem.r8",PopRefPopI,PushR8,InlineNone,IObjModel,1,0xFF,0x99,Next)
    OPDEF(LDELEM_REF,"ldelem.ref",PopRefPopI,PushRef,InlineNone,IObjModel,1,0xFF,0x9A,Next)
    OPDEF(STELEM_I,"stelem.i",PopRefPopIPopI,Push0,InlineNone,IObjModel,1,0xFF,0x9B,Next)
    OPDEF(STELEM_I1,"stelem.i1",PopRefPopIPopI,Push0,InlineNone,IObjModel,1,0xFF,0x9C,Next)
    OPDEF(STELEM_I2,"stelem.i2",PopRefPopIPopI,Push0,InlineNone,IObjModel,1,0xFF,0x9D,Next)
    OPDEF(STELEM_I4,"stelem.i4",PopRefPopIPopI,Push0,InlineNone,IObjModel,1,0xFF,0x9E,Next)
    OPDEF(STELEM_I8,"stelem.i8",PopRefPopIPopI8,Push0,InlineNone,IObjModel,1,0xFF,0x9F,Next)
    OPDEF(STELEM_R4,"stelem.r4",PopRefPopIPopR4,Push0,InlineNone,IObjModel,1,0xFF,0xA0,Next)
    OPDEF(STELEM_R8,"stelem.r8",PopRefPopIPopR8,Push0,InlineNone,IObjModel,1,0xFF,0xA1,Next)
    OPDEF(STELEM_REF,"stelem.ref",PopRefPopIPopRef,Push0,InlineNone,IObjModel,1,0xFF,0xA2,Next)
    OPDEF(UNUSED2,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xA3,Next)
    OPDEF(CONV_OVF_I1,"conv.ovf.i1",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xB3,Next)
    OPDEF(CONV_OVF_U1,"conv.ovf.u1",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xB4,Next)
    OPDEF(CONV_OVF_I2,"conv.ovf.i2",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xB5,Next)
    OPDEF(CONV_OVF_U2,"conv.ovf.u2",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xB6,Next)
    OPDEF(CONV_OVF_I4,"conv.ovf.i4",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xB7,Next)
    OPDEF(CONV_OVF_U4,"conv.ovf.u4",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xB8,Next)
    OPDEF(CONV_OVF_I8,"conv.ovf.i8",Pop1,PushI8,InlineNone,IPrimitive,1,0xFF,0xB9,Next)
    OPDEF(CONV_OVF_U8,"conv.ovf.u8",Pop1,PushI8,InlineNone,IPrimitive,1,0xFF,0xBA,Next)
    OPDEF(UNUSED50,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xBB,Next)
    OPDEF(UNUSED18,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xBC,Next)
    OPDEF(UNUSED19,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xBD,Next)
    OPDEF(UNUSED20,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xBE,Next)
    OPDEF(UNUSED21,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xBF,Next)
    OPDEF(UNUSED22,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xC0,Next)
    OPDEF(UNUSED23,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xC1,Next)
    OPDEF(REFANYVAL,"refanyval",Pop1,PushI,InlineType,IPrimitive,1,0xFF,0xC2,Next)
    OPDEF(CKFINITE,"ckfinite",Pop1,PushR8,InlineNone,IPrimitive,1,0xFF,0xC3,Next)
    OPDEF(UNUSED24,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xC4,Next)
    OPDEF(UNUSED25,"unused",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xC5,Next)
    OPDEF(MKREFANY,"mkrefany",PopI,Push1,InlineType,IPrimitive,1,0xFF,0xC6,Next)
    OPDEF(LDTOKEN,"ldtoken",Pop0,PushI,InlineTok,IPrimitive,1,0xFF,0xD0,Next)
    OPDEF(CONV_U2,"conv.u2",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xD1,Next)
    OPDEF(CONV_U1,"conv.u1",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xD2,Next)
    OPDEF(CONV_I,"conv.i",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xD3,Next)
    OPDEF(CONV_OVF_I,"conv.ovf.i",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xD4,Next)
    OPDEF(CONV_OVF_U,"conv.ovf.u",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xD5,Next)
    OPDEF(ADD_OVF,"add.ovf",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0xD6,Next)
    OPDEF(ADD_OVF_UN,"add.ovf.un",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0xD7,Next)
    OPDEF(MUL_OVF,"mul.ovf",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0xD8,Next)
    OPDEF(MUL_OVF_UN,"mul.ovf.un",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0xD9,Next)
    OPDEF(SUB_OVF,"sub.ovf",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0xDA,Next)
    OPDEF(SUB_OVF_UN,"sub.ovf.un",Pop1Pop1,Push1,InlineNone,IPrimitive,1,0xFF,0xDB,Next)
    OPDEF(ENDFINALLY,"endfinally",Pop0,Push0,InlineNone,IPrimitive,1,0xFF,0xDC,Return)
    OPDEF(LEAVE,"leave",Pop0,Push0,InlineBrTarget,IPrimitive,1,0xFF,0xDD,Branch)
    OPDEF(LEAVE_S,"leave.s",Pop0,Push0,ShortInlineBrTarget,IPrimitive,1,0xFF,0xDE,Branch)
    OPDEF(STIND_I,"stind.i",PopIPopI,Push0,InlineNone,IPrimitive,1,0xFF,0xDF,Next)
    OPDEF(CONV_U,"conv.u",Pop1,PushI,InlineNone,IPrimitive,1,0xFF,0xE0,Next)
    OPDEF(PREFIX7,"prefix7",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xF8,Meta)
    OPDEF(PREFIX6,"prefix6",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xF9,Meta)
    OPDEF(PREFIX5,"prefix5",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xFA,Meta)
    OPDEF(PREFIX4,"prefix4",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xFB,Meta)
    OPDEF(PREFIX3,"prefix3",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xFC,Meta)
    OPDEF(PREFIX2,"prefix2",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xFD,Meta)
    OPDEF(PREFIX1,"prefix1",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xFE,Meta)
    OPDEF(PREFIXREF,"prefixref",Pop0,Push0,InlineNone,IInternal,1,0xFF,0xFF,Meta)
    OPDEF(ARGLIST,"arglist",Pop0,PushI,InlineNone,IPrimitive,2,0xFE,0x00,Next)
    OPDEF(CEQ,"ceq",Pop1Pop1,PushI,InlineNone,IPrimitive,2,0xFE,0x01,Next)
    OPDEF(CGT,"cgt",Pop1Pop1,PushI,InlineNone,IPrimitive,2,0xFE,0x02,Next)
    OPDEF(CGT_UN,"cgt.un",Pop1Pop1,PushI,InlineNone,IPrimitive,2,0xFE,0x03,Next)
    OPDEF(CLT,"clt",Pop1Pop1,PushI,InlineNone,IPrimitive,2,0xFE,0x04,Next)
    OPDEF(CLT_UN,"clt.un",Pop1Pop1,PushI,InlineNone,IPrimitive,2,0xFE,0x05,Next)
    OPDEF(LDFTN,"ldftn",Pop0,PushI,InlineMethod,IPrimitive,2,0xFE,0x06,Next)
    OPDEF(LDVIRTFTN,"ldvirtftn",PopRef,PushI,InlineMethod,IPrimitive,2,0xFE,0x07,Next)
    OPDEF(LDARG,"ldarg",Pop0,Push1,InlineVar,IPrimitive,2,0xFE,0x09,Next)
    OPDEF(LDARGA,"ldarga",Pop0,PushI,InlineVar,IPrimitive,2,0xFE,0x0A,Next)
    OPDEF(STARG,"starg",Pop1,Push0,InlineVar,IPrimitive,2,0xFE,0x0B,Next)
    OPDEF(LDLOC,"ldloc",Pop0,Push1,InlineVar,IPrimitive,2,0xFE,0x0C,Next)
    OPDEF(LDLOCA,"ldloca",Pop0,PushI,InlineVar,IPrimitive,2,0xFE,0x0D,Next)
    OPDEF(STLOC,"stloc",Pop1,Push0,InlineVar,IPrimitive,2,0xFE,0x0E,Next)
    OPDEF(LOCALLOC,"localloc",PopI,PushI,InlineNone,IPrimitive,2,0xFE,0x0F,Next)
    OPDEF(ENDFILTER,"endfilter",PopI,Push0,InlineNone,IPrimitive,2,0xFE,0x11,Return)
    OPDEF(UNALIGNED,"unaligned.",Pop0,Push0,ShortInlineI,IPrefix,2,0xFE,0x12,Meta)
    OPDEF(VOLATILE,"volatile.",Pop0,Push0,InlineNone,IPrefix,2,0xFE,0x13,Meta)
    OPDEF(TAILCALL,"tail.",Pop0,Push0,InlineNone,IPrefix,2,0xFE,0x14,Meta)
    OPDEF(INITOBJ,"initobj",PopI,Push0,InlineType,IObjModel,2,0xFE,0x15,Next)
    OPDEF(CPBLK,"cpblk",PopIPopIPopI,Push0,InlineNone,IPrimitive,2,0xFE,0x17,Next)
    OPDEF(INITBLK,"initblk",PopIPopIPopI,Push0,InlineNone,IPrimitive,2,0xFE,0x18,Next)
    OPDEF(SIZEOF,"sizeof",Pop0,PushI,InlineType,IPrimitive,2,0xFE,0x1C,Next)
    OPDEF(REFANYTYPE,"refanytype",Pop1,PushI,InlineNone,IPrimitive,2,0xFE,0x1D,Next)
);

