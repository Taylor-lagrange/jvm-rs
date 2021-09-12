use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct IOR {}
pub struct LOR {}

impl NoOperandsInstruction for IOR {}
impl NoOperandsInstruction for LOR {}

impl Instruction for IOR {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v1 = frame.operand_stack.pop_int();
        let v2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(v1 | v2);
    }
}

impl Instruction for LOR {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v1 = frame.operand_stack.pop_long();
        let v2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(v1 | v2);
    }
}
