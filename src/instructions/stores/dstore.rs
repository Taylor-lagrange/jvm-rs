use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DSTORE {}
pub struct DSTORE_0 {}
pub struct DSTORE_1 {}
pub struct DSTORE_2 {}
pub struct DSTORE_3 {}

impl Index8Instruction for DSTORE {}
impl NoOperandsInstruction for DSTORE_0 {}
impl NoOperandsInstruction for DSTORE_1 {}
impl NoOperandsInstruction for DSTORE_2 {}
impl NoOperandsInstruction for DSTORE_3 {}

fn dstore(frame: &mut Frame, index: usize) {
  let val = frame.operand_stack.pop_double();
  frame.local_vars.set_double(index, val);
}

impl Instruction for DSTORE {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    dstore(frame, index);
  }
}

impl Instruction for DSTORE_0 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    dstore(frame, 0);
  }
}

impl Instruction for DSTORE_1 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    dstore(frame, 1);
  }
}

impl Instruction for DSTORE_2 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    dstore(frame, 2);
  }
}

impl Instruction for DSTORE_3 {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    dstore(frame, 3);
  }
}
