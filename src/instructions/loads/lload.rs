use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct LLOAD {}
pub struct LLOAD_0 {}
pub struct LLOAD_1 {}
pub struct LLOAD_2 {}
pub struct LLOAD_3 {}

impl Index8Instruction for LLOAD {}
impl NoOperandsInstruction for LLOAD_0 {}
impl NoOperandsInstruction for LLOAD_1 {}
impl NoOperandsInstruction for LLOAD_2 {}
impl NoOperandsInstruction for LLOAD_3 {}

fn lload(frame: &mut Frame, index: usize) {
  frame
    .operand_stack
    .push_long(frame.local_vars.get_long(index))
}

impl Instruction for LLOAD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    lload(frame, index);
  }
}

impl Instruction for LLOAD_0 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    lload(frame, 0);
  }
}

impl Instruction for LLOAD_1 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    lload(frame, 1);
  }
}

impl Instruction for LLOAD_2 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    lload(frame, 2);
  }
}

impl Instruction for LLOAD_3 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    lload(frame, 3);
  }
}
