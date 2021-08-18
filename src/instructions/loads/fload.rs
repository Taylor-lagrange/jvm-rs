use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct FLOAD {}
pub struct FLOAD_0 {}
pub struct FLOAD_1 {}
pub struct FLOAD_2 {}
pub struct FLOAD_3 {}

impl Index8Instruction for FLOAD {}
impl NoOperandsInstruction for FLOAD_0 {}
impl NoOperandsInstruction for FLOAD_1 {}
impl NoOperandsInstruction for FLOAD_2 {}
impl NoOperandsInstruction for FLOAD_3 {}

fn fload(frame: &mut Frame, index: usize) {
  frame
    .operand_stack
    .push_float(frame.local_vars.get_float(index))
}

impl Instruction for FLOAD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    fload(frame, index);
  }
}

impl Instruction for FLOAD_0 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    fload(frame, 0);
  }
}

impl Instruction for FLOAD_1 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    fload(frame, 1);
  }
}

impl Instruction for FLOAD_2 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    fload(frame, 2);
  }
}

impl Instruction for FLOAD_3 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    fload(frame, 3);
  }
}
