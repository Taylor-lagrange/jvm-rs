use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ISHL {}
pub struct ISHR {}
pub struct IUSHR {}
pub struct LSHL {}
pub struct LSHR {}
pub struct LUSHR {}

impl NoOperandsInstruction for ISHL {} // int左位移
impl NoOperandsInstruction for ISHR {} // int算术右位移
impl NoOperandsInstruction for IUSHR {} // int逻辑右位移
impl NoOperandsInstruction for LSHL {} // long左位移
impl NoOperandsInstruction for LSHR {} // long算术右位移
impl NoOperandsInstruction for LUSHR {} // long逻辑右位移

impl Instruction for ISHL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(v1 << ((v2 as u32) & 0x1f));
    }
}

impl Instruction for ISHR {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(v1 >> ((v2 as u32) & 0x1f));
    }
}

impl Instruction for IUSHR {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        frame
            .operand_stack
            .push_int(((v1 as u32) >> ((v2 as u32) & 0x1f)) as i32);
    }
}

impl Instruction for LSHL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(v1 << ((v2 as u32) & 0x3f));
    }
}

impl Instruction for LSHR {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(v1 >> ((v2 as u32) & 0x3f));
    }
}

impl Instruction for LUSHR {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_long();
        frame
            .operand_stack
            .push_long(((v1 as u64) >> ((v2 as u32) & 0x3f)) as i64);
    }
}
