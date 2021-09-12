use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DSUB {}
pub struct FSUB {}
pub struct ISUB {}
pub struct LSUB {}

impl NoOperandsInstruction for DSUB {}
impl NoOperandsInstruction for FSUB {}
impl NoOperandsInstruction for ISUB {}
impl NoOperandsInstruction for LSUB {}

impl Instruction for DSUB {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_double();
        let v1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(v1 - v2);
    }
}

impl Instruction for FSUB {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_float();
        let v1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(v1 - v2);
    }
}

impl Instruction for ISUB {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(v1 - v2);
    }
}

impl Instruction for LSUB {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_long();
        let v1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(v1 - v2);
    }
}
