use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;

pub struct ARRAY_LENGTH {}

impl NoOperandsInstruction for ARRAY_LENGTH {}

impl Instruction for ARRAY_LENGTH {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let arr_array = frame.operand_stack.pop_ref();
    if arr_array.is_none() {
      panic!("java.lang.NullPointerException");
    }
    frame
      .operand_stack
      .push_int(arr_array.unwrap().borrow().array_length() as i32);
  }
}
