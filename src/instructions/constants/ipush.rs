use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct BIPUSH {} // Push byte
pub struct SIPUSH {} // Push short

impl Instruction for BIPUSH {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    frame.operand_stack.push_int(reader.read_i8() as i32);
    frame.next_pc = reader.pc;
  }
}
impl Instruction for SIPUSH {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    frame.operand_stack.push_int(reader.read_i16() as i32);
    frame.next_pc = reader.pc;
  }
}
