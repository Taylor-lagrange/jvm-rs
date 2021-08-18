use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct POP {}
pub struct POP2 {}

impl NoOperandsInstruction for POP {}
impl NoOperandsInstruction for POP2 {}

impl Instruction for POP {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    frame.operand_stack.pop_slot();
  }
}

impl Instruction for POP2 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    frame.operand_stack.pop_slot();
    frame.operand_stack.pop_slot();
  }
}
