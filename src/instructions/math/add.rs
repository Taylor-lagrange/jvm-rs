use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DADD {}
pub struct FADD {}
pub struct IADD {}
pub struct LADD {}

impl NoOperandsInstruction for DADD {}
impl NoOperandsInstruction for FADD {}
impl NoOperandsInstruction for IADD {}
impl NoOperandsInstruction for LADD {}

impl Instruction for DADD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_double();
    let v2 = frame.operand_stack.pop_double();
    frame.operand_stack.push_double(v1 + v2);
  }
}

impl Instruction for FADD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_float();
    let v2 = frame.operand_stack.pop_float();
    frame.operand_stack.push_float(v1 + v2);
  }
}

impl Instruction for IADD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_int();
    let v2 = frame.operand_stack.pop_int();
    frame.operand_stack.push_int(v1 + v2);
  }
}

impl Instruction for LADD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v1 = frame.operand_stack.pop_long();
    let v2 = frame.operand_stack.pop_long();
    frame.operand_stack.push_long(v1 + v2);
  }
}
