use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DMUL {}
pub struct FMUL {}
pub struct IMUL {}
pub struct LMUL {}

impl NoOperandsInstruction for DMUL {}
impl NoOperandsInstruction for FMUL {}
impl NoOperandsInstruction for IMUL {}
impl NoOperandsInstruction for LMUL {}

impl Instruction for DMUL {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_double();
    let v2 = frame.operand_stack.pop_double();
    frame.operand_stack.push_double(v1 * v2);
  }
}

impl Instruction for FMUL {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_float();
    let v2 = frame.operand_stack.pop_float();
    frame.operand_stack.push_float(v1 * v2);
  }
}

impl Instruction for IMUL {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_int();
    let v2 = frame.operand_stack.pop_int();
    frame.operand_stack.push_int(v1 * v2);
  }
}

impl Instruction for LMUL {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_long();
    let v2 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(v1 * v2);
  }
}
