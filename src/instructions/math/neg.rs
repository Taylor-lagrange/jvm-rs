use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DNEG {}
pub struct FNEG {}
pub struct INEG {}
pub struct LNEG {}

impl NoOperandsInstruction for DNEG {}
impl NoOperandsInstruction for FNEG {}
impl NoOperandsInstruction for INEG {}
impl NoOperandsInstruction for LNEG {}

impl Instruction for DNEG {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(-v);
    }
}

impl Instruction for FNEG {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(-v);
    }
}

impl Instruction for INEG {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(-v);
    }
}

impl Instruction for LNEG {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(-v);
    }
}
