use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct IINC {}

impl NoOperandsInstruction for IINC {}

impl Instruction for IINC {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = reader.read_u8() as usize;
    let consts = reader.read_i8() as i32;
    frame.next_pc = reader.pc;
    let mut val = frame.local_vars.get_int(index);
    val += consts;
    frame.local_vars.set_int(index, val);
  }
}
