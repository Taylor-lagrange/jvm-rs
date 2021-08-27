use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct INVOKE_SPECIAL {}

impl Index16Instruction for INVOKE_SPECIAL {}

impl Instruction for INVOKE_SPECIAL {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    self.fetch_operands(reader, frame);
    frame.operand_stack.pop_ref();
  }
}
