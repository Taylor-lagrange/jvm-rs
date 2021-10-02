use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ARRAY_LENGTH {}

impl NoOperandsInstruction for ARRAY_LENGTH {}

impl Instruction for ARRAY_LENGTH {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let arr_array = frame.operand_stack.pop_ref();
        if arr_array.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let len = arr_array.unwrap().borrow().array_length();
        frame.operand_stack.push_int(len as i32);
    }
}
