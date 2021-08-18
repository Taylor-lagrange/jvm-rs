use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct IF_ACMPEQ {}
pub struct IF_ACMPNE {}

impl BranchInstruction for IF_ACMPEQ {}
impl BranchInstruction for IF_ACMPNE {}

impl Instruction for IF_ACMPEQ {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let offset = self.fetch_operands(reader, frame);
    let v2 = frame.operand_stack.pop_ref();
    let v1 = frame.operand_stack.pop_ref();
    if (v1.is_none() && v2.is_none()) || std::ptr::eq(v1.unwrap(), v2.unwrap()) {
      branch(frame, offset);
    }
  }
}

impl Instruction for IF_ACMPNE {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let offset = self.fetch_operands(reader, frame);
    let v2 = frame.operand_stack.pop_ref();
    let v1 = frame.operand_stack.pop_ref();
    if !((v1.is_none() && v2.is_none()) || std::ptr::eq(v1.unwrap(), v2.unwrap())) {
      branch(frame, offset);
    }
  }
}
