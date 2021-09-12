use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct F2D {}
pub struct F2I {}
pub struct F2L {}

impl NoOperandsInstruction for F2D {}
impl NoOperandsInstruction for F2I {}
impl NoOperandsInstruction for F2L {}

impl Instruction for F2D {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_float();
        frame.operand_stack.push_double(v as f64);
    }
}

impl Instruction for F2I {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_float();
        frame.operand_stack.push_int(v as i32);
    }
}

impl Instruction for F2L {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_float();
        frame.operand_stack.push_long(v as i64);
    }
}
