use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct L2D {}
pub struct L2F {}
pub struct L2I {}

impl NoOperandsInstruction for L2D {}
impl NoOperandsInstruction for L2F {}
impl NoOperandsInstruction for L2I {}

impl Instruction for L2D {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_long();
        frame.operand_stack.push_double(v as f64);
    }
}

impl Instruction for L2F {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_long();
        frame.operand_stack.push_float(v as f32);
    }
}

impl Instruction for L2I {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v = frame.operand_stack.pop_long();
        frame.operand_stack.push_int(v as i32);
    }
}
