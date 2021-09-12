use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DDIV {}
pub struct FDIV {}
pub struct IDIV {}
pub struct LDIV {}

impl NoOperandsInstruction for DDIV {}
impl NoOperandsInstruction for FDIV {}
impl NoOperandsInstruction for IDIV {}
impl NoOperandsInstruction for LDIV {}

impl Instruction for DDIV {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_double();
        let v1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(v1 / v2);
    }
}

impl Instruction for FDIV {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_float();
        let v1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(v1 / v2);
    }
}

impl Instruction for IDIV {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        frame.operand_stack.push_int(v1 / v2);
    }
}

impl Instruction for LDIV {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_long();
        let v1 = frame.operand_stack.pop_long();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        frame.operand_stack.push_long(v1 / v2);
    }
}
