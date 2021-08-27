use super::base::instruction::*;
use super::comparisons::*;
use super::constants::*;
use super::control::*;
use super::conversions::*;
use super::extended::*;
use super::loads::*;
use super::math::*;
use super::reference::*;
use super::stack::*;
use super::stores::*;

pub fn new_instruction(opcode: u8) -> Box<dyn Instruction> {
  match opcode {
    0x00 => Box::new(nop::Nop {}),
    0x01 => Box::new(consts::ACONST_NULL {}),
    0x02 => Box::new(consts::ICONST_M1 {}),

    0x03 => Box::new(consts::ICONST_0 {}),
    0x04 => Box::new(consts::ICONST_1 {}),
    0x05 => Box::new(consts::ICONST_2 {}),
    0x06 => Box::new(consts::ICONST_3 {}),
    0x07 => Box::new(consts::ICONST_4 {}),
    0x08 => Box::new(consts::ICONST_5 {}),

    0x09 => Box::new(consts::LCONST_0 {}),
    0x0a => Box::new(consts::LCONST_1 {}),

    0x0b => Box::new(consts::FCONST_0 {}),
    0x0c => Box::new(consts::FCONST_1 {}),
    0x0d => Box::new(consts::FCONST_2 {}),

    0x0e => Box::new(consts::DCONST_0 {}),
    0x0f => Box::new(consts::DCONST_1 {}),

    0x10 => Box::new(ipush::BIPUSH {}),
    0x11 => Box::new(ipush::SIPUSH {}),

    0x12 => Box::new(ldc::LDC {}),
    0x13 => Box::new(ldc::LDC_W {}),
    0x14 => Box::new(ldc::LDC2_W {}),

    0x15 => Box::new(iload::ILOAD {}),
    0x16 => Box::new(lload::LLOAD {}),
    0x17 => Box::new(fload::FLOAD {}),
    0x18 => Box::new(dload::DLOAD {}),
    0x19 => Box::new(aload::ALOAD {}),

    0x1a => Box::new(iload::ILOAD_0 {}),
    0x1b => Box::new(iload::ILOAD_1 {}),
    0x1c => Box::new(iload::ILOAD_2 {}),
    0x1d => Box::new(iload::ILOAD_3 {}),

    0x1e => Box::new(lload::LLOAD_0 {}),
    0x1f => Box::new(lload::LLOAD_1 {}),
    0x20 => Box::new(lload::LLOAD_2 {}),
    0x21 => Box::new(lload::LLOAD_3 {}),

    0x22 => Box::new(fload::FLOAD_0 {}),
    0x23 => Box::new(fload::FLOAD_1 {}),
    0x24 => Box::new(fload::FLOAD_2 {}),
    0x25 => Box::new(fload::FLOAD_3 {}),

    0x26 => Box::new(dload::DLOAD_0 {}),
    0x27 => Box::new(dload::DLOAD_1 {}),
    0x28 => Box::new(dload::DLOAD_2 {}),
    0x29 => Box::new(dload::DLOAD_3 {}),

    0x2a => Box::new(aload::ALOAD_0 {}),
    0x2b => Box::new(aload::ALOAD_1 {}),
    0x2c => Box::new(aload::ALOAD_2 {}),
    0x2d => Box::new(aload::ALOAD_3 {}),

    // 0x2e => iaload,
    // 0x2f => laload,
    // 0x30 => faload,
    // 0x31 => daload,
    // 0x32 => aaload,
    // 0x33 => baload,
    // 0x34 => caload,
    // 0x35 => saload,
    0x36 => Box::new(istore::ISTORE {}),
    0x37 => Box::new(lstore::LSTORE {}),
    0x38 => Box::new(fstore::FSTORE {}),
    0x39 => Box::new(dstore::DSTORE {}),
    0x3a => Box::new(astore::ASTORE {}),

    0x3b => Box::new(istore::ISTORE_0 {}),
    0x3c => Box::new(istore::ISTORE_1 {}),
    0x3d => Box::new(istore::ISTORE_2 {}),
    0x3e => Box::new(istore::ISTORE_3 {}),

    0x3f => Box::new(lstore::LSTORE_0 {}),
    0x40 => Box::new(lstore::LSTORE_1 {}),
    0x41 => Box::new(lstore::LSTORE_2 {}),
    0x42 => Box::new(lstore::LSTORE_3 {}),

    0x43 => Box::new(fstore::FSTORE_0 {}),
    0x44 => Box::new(fstore::FSTORE_1 {}),
    0x45 => Box::new(fstore::FSTORE_2 {}),
    0x46 => Box::new(fstore::FSTORE_3 {}),

    0x47 => Box::new(dstore::DSTORE_0 {}),
    0x48 => Box::new(dstore::DSTORE_1 {}),
    0x49 => Box::new(dstore::DSTORE_2 {}),
    0x4a => Box::new(dstore::DSTORE_3 {}),

    0x4b => Box::new(astore::ASTORE_0 {}),
    0x4c => Box::new(astore::ASTORE_1 {}),
    0x4d => Box::new(astore::ASTORE_2 {}),
    0x4e => Box::new(astore::ASTORE_3 {}),

    // 0x4f => iastore,
    // 0x50 => lastore,
    // 0x51 => fastore,
    // 0x52 => dastore,
    // 0x53 => aastore,
    // 0x54 => bastore,
    // 0x55 => castore,
    // 0x56 => sastore,
    0x57 => Box::new(pop::POP {}),
    0x58 => Box::new(pop::POP2 {}),
    0x59 => Box::new(dup::DUP {}),
    0x5a => Box::new(dup::DUP_X1 {}),
    0x5b => Box::new(dup::DUP_X2 {}),
    0x5c => Box::new(dup::DUP2 {}),
    0x5d => Box::new(dup::DUP2_X1 {}),
    0x5e => Box::new(dup::DUP2_X2 {}),
    0x5f => Box::new(swap::SWAP {}),

    0x60 => Box::new(add::IADD {}),
    0x61 => Box::new(add::LADD {}),
    0x62 => Box::new(add::FADD {}),
    0x63 => Box::new(add::DADD {}),

    0x64 => Box::new(sub::ISUB {}),
    0x65 => Box::new(sub::LSUB {}),
    0x66 => Box::new(sub::FSUB {}),
    0x67 => Box::new(sub::DSUB {}),

    0x68 => Box::new(mul::IMUL {}),
    0x69 => Box::new(mul::LMUL {}),
    0x6a => Box::new(mul::FMUL {}),
    0x6b => Box::new(mul::DMUL {}),

    0x6c => Box::new(div::IDIV {}),
    0x6d => Box::new(div::LDIV {}),
    0x6e => Box::new(div::FDIV {}),
    0x6f => Box::new(div::DDIV {}),

    0x70 => Box::new(rem::IREM {}),
    0x71 => Box::new(rem::LREM {}),
    0x72 => Box::new(rem::FREM {}),
    0x73 => Box::new(rem::DREM {}),

    0x74 => Box::new(neg::INEG {}),
    0x75 => Box::new(neg::LNEG {}),
    0x76 => Box::new(neg::FNEG {}),
    0x77 => Box::new(neg::DNEG {}),

    0x78 => Box::new(sh::ISHL {}),
    0x79 => Box::new(sh::LSHL {}),
    0x7a => Box::new(sh::ISHR {}),
    0x7b => Box::new(sh::LSHR {}),
    0x7c => Box::new(sh::IUSHR {}),
    0x7d => Box::new(sh::LUSHR {}),

    0x7e => Box::new(and::IAND {}),
    0x7f => Box::new(and::LAND {}),

    0x80 => Box::new(or::IOR {}),
    0x81 => Box::new(or::LOR {}),

    0x82 => Box::new(xor::IXOR {}),
    0x83 => Box::new(xor::LXOR {}),

    0x84 => Box::new(iinc::IINC {}),

    0x85 => Box::new(i2x::I2L {}),
    0x86 => Box::new(i2x::I2F {}),
    0x87 => Box::new(i2x::I2D {}),

    0x88 => Box::new(l2x::L2I {}),
    0x89 => Box::new(l2x::L2F {}),
    0x8a => Box::new(l2x::L2D {}),

    0x8b => Box::new(f2x::F2I {}),
    0x8c => Box::new(f2x::F2L {}),
    0x8d => Box::new(f2x::F2D {}),

    0x8e => Box::new(d2x::D2I {}),
    0x8f => Box::new(d2x::D2L {}),
    0x90 => Box::new(d2x::D2F {}),

    0x91 => Box::new(i2x::I2B {}),
    0x92 => Box::new(i2x::I2C {}),
    0x93 => Box::new(i2x::I2S {}),

    0x94 => Box::new(lcmp::LCMP {}),
    0x95 => Box::new(fcmp::FCMPL {}),
    0x96 => Box::new(fcmp::FCMPG {}),
    0x97 => Box::new(dcmp::DCMPL {}),
    0x98 => Box::new(dcmp::DCMPG {}),

    0x99 => Box::new(ifcond::IFEQ {}),
    0x9a => Box::new(ifcond::IFNE {}),
    0x9b => Box::new(ifcond::IFLT {}),
    0x9c => Box::new(ifcond::IFGE {}),
    0x9d => Box::new(ifcond::IFGT {}),
    0x9e => Box::new(ifcond::IFLE {}),

    0x9f => Box::new(if_icmp::IF_ICMPEQ {}),
    0xa0 => Box::new(if_icmp::IF_ICMPNE {}),
    0xa1 => Box::new(if_icmp::IF_ICMPLT {}),
    0xa2 => Box::new(if_icmp::IF_ICMPGE {}),
    0xa3 => Box::new(if_icmp::IF_ICMPGT {}),
    0xa4 => Box::new(if_icmp::IF_ICMPLE {}),

    0xa5 => Box::new(if_acmp::IF_ACMPEQ {}),
    0xa6 => Box::new(if_acmp::IF_ACMPNE {}),

    0xa7 => Box::new(goto::GOTO {}),

    // 0xa8 => JSR,
    // 0xa9 => RET,
    0xaa => Box::new(tableswitch::TABLE_SWITCH {}),
    0xab => Box::new(lookupswitch::LOOKUP_SWITCH {}),

    //  0xac => ireturn
    //  0xad => lreturn
    //  0xae => freturn
    //  0xaf => dreturn
    //  0xb0 => areturn
    //  0xb1 => _return
    0xb2 => Box::new(getstatic::GET_STATIC {}),
    0xb3 => Box::new(putstatic::PUT_STATIC {}),
    0xb4 => Box::new(getfield::GET_FIELD {}),
    0xb5 => Box::new(putfield::PUT_FIELD {}),
    0xb6 => Box::new(invokevirtual::INVOKE_VIRTUAL {}),
    0xb7 => Box::new(invokespecial::INVOKE_SPECIAL {}),
    //  0xb8 => INVOKE_STATIC
    //  0xb9 => INVOKE_INTERFACE
    //  0xba => INVOKE_DYNAMIC
    0xbb => Box::new(new::NEW {}),
    //  0xbc => NEW_ARRAY
    //  0xbd => ANEW_ARRAY
    //  0xbe => arraylength
    //  0xbf => athrow
    0xc0 => Box::new(checkcast::CHECK_CAST {}),
    0xc1 => Box::new(instanceof::INSTANCE_OF {}),
    //  0xc2 => monitorenter
    //  0xc3 => monitorexit
    //  0xc4 => WIDE
    //  0xc5 => MULTI_ANEW_ARRAY
    0xc6 => Box::new(ifnull::IFNULL {}),
    0xc7 => Box::new(ifnull::IFNONNULL {}),
    0xc8 => Box::new(goto_w::GOTO_W {}),
    // 0xc9: JSR_W
    // 0xca: breakpoint
    // 0xfe: impdep1
    // 0xff: impdep2
    _ => panic!("Unsupported opcode: 0x{:X}", opcode),
  }
}
