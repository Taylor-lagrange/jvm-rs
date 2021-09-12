use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;

pub struct AALOAD {}
pub struct BALOAD {}
pub struct CALOAD {}
pub struct DALOAD {}
pub struct FALOAD {}
pub struct IALOAD {}
pub struct LALOAD {}
pub struct SALOAD {}

impl NoOperandsInstruction for AALOAD {}
impl NoOperandsInstruction for BALOAD {}
impl NoOperandsInstruction for CALOAD {}
impl NoOperandsInstruction for DALOAD {}
impl NoOperandsInstruction for FALOAD {}
impl NoOperandsInstruction for IALOAD {}
impl NoOperandsInstruction for LALOAD {}
impl NoOperandsInstruction for SALOAD {}

impl Instruction for AALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayRefs(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame
                .operand_stack
                .push_ref(Some(array[index as usize].clone()))
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for BALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayBytes(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_int(array[index as usize] as i32)
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for CALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayChars(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_int(array[index as usize] as i32)
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for DALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayDoubles(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_double(array[index as usize])
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for FALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayFloats(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_float(array[index as usize])
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for IALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayInts(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_int(array[index as usize])
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for LALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayLongs(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_long(array[index as usize])
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}

impl Instruction for SALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let arr_ref = frame.operand_stack.pop_ref();
        if arr_ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let rc = arr_ref.unwrap();
        let refs = rc.borrow();
        if let ObjectData::ArrayShorts(array) = &refs.data {
            if !(0 <= index && index < array.len() as i32) {
                panic!("ArrayIndexOutOfBoundsException");
            }
            frame.operand_stack.push_int(array[index as usize] as i32)
        } else {
            panic!("data type in object is nor expect!");
        }
    }
}
