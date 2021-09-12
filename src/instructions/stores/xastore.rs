use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;
use std::rc::Rc;

pub struct AASTORE {}
pub struct BASTORE {}
pub struct CASTORE {}
pub struct DASTORE {}
pub struct FASTORE {}
pub struct IASTORE {}
pub struct LASTORE {}
pub struct SASTORE {}

impl NoOperandsInstruction for AASTORE {}
impl NoOperandsInstruction for BASTORE {}
impl NoOperandsInstruction for CASTORE {}
impl NoOperandsInstruction for DASTORE {}
impl NoOperandsInstruction for FASTORE {}
impl NoOperandsInstruction for IASTORE {}
impl NoOperandsInstruction for LASTORE {}
impl NoOperandsInstruction for SASTORE {}

impl Instruction for AASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_ref();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayRefs(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val.unwrap();
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for BASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_int();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayBytes(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val as i8;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for CASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_int();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayChars(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val as u16;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for DASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_double();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayDoubles(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for FASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_float();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayFloats(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for IASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_int();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayInts(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for LASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_long();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayLongs(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for SASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let val = frame.operand_stack.pop_int();
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let mut refs = rc.borrow_mut();
        if let ObjectData::ArrayShorts(array) = &mut refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            array[index as usize] = val as i16;
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}
