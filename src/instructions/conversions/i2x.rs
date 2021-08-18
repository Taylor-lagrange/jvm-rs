use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct I2B {}
pub struct I2C {}
pub struct I2S {}
pub struct I2L {}
pub struct I2F {}
pub struct I2D {}

impl NoOperandsInstruction for I2B {}
impl NoOperandsInstruction for I2C {}
impl NoOperandsInstruction for I2S {}
impl NoOperandsInstruction for I2L {}
impl NoOperandsInstruction for I2F {}
impl NoOperandsInstruction for I2D {}

impl Instruction for I2B {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v = frame.operand_stack.pop_int();
    frame.operand_stack.push_int((v as i8) as i32);
  }
}

impl Instruction for I2C {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v = frame.operand_stack.pop_int();
    frame.operand_stack.push_int((v as u16) as i32);
  }
}

impl Instruction for I2S {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v = frame.operand_stack.pop_int();
    frame.operand_stack.push_int((v as i16) as i32);
  }
}

impl Instruction for I2L {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v = frame.operand_stack.pop_int();
    frame.operand_stack.push_long(v as i64);
  }
}

impl Instruction for I2F {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v = frame.operand_stack.pop_int();
    frame.operand_stack.push_float(v as f32);
  }
}

impl Instruction for I2D {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let v = frame.operand_stack.pop_int();
    frame.operand_stack.push_double(v as f64);
  }
}
