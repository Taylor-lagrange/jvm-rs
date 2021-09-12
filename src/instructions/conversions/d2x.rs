use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct D2F {}
pub struct D2I {}
pub struct D2L {}

impl NoOperandsInstruction for D2F {}
impl NoOperandsInstruction for D2I {}
impl NoOperandsInstruction for D2L {}

impl Instruction for D2F {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_double();
        frame.operand_stack.push_float(v as f32);
    }
}

impl Instruction for D2I {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_double();
        frame.operand_stack.push_int(v as i32);
    }
}

impl Instruction for D2L {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_double();
        frame.operand_stack.push_long(v as i64);
    }
}
