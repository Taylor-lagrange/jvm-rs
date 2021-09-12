use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DREM {}
pub struct FREM {}
pub struct IREM {}
pub struct LREM {}

impl NoOperandsInstruction for DREM {}
impl NoOperandsInstruction for FREM {}
impl NoOperandsInstruction for IREM {}
impl NoOperandsInstruction for LREM {}

impl Instruction for DREM {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_double();
        let v1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(v1 % v2);
    }
}

impl Instruction for FREM {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_float();
        let v1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(v1 % v2);
    }
}

impl Instruction for IREM {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        frame.operand_stack.push_int(v1 % v2);
    }
}

impl Instruction for LREM {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_long();
        let v1 = frame.operand_stack.pop_long();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        frame.operand_stack.push_long(v1 % v2);
    }
}
