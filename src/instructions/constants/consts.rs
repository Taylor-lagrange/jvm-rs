use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ACONST_NULL {}
pub struct DCONST_0 {}
pub struct DCONST_1 {}
pub struct FCONST_0 {}
pub struct FCONST_1 {}
pub struct FCONST_2 {}
pub struct ICONST_M1 {}
pub struct ICONST_0 {}
pub struct ICONST_1 {}
pub struct ICONST_2 {}
pub struct ICONST_3 {}
pub struct ICONST_4 {}
pub struct ICONST_5 {}
pub struct LCONST_0 {}
pub struct LCONST_1 {}

impl NoOperandsInstruction for ACONST_NULL {}
impl NoOperandsInstruction for DCONST_0 {}
impl NoOperandsInstruction for DCONST_1 {}
impl NoOperandsInstruction for FCONST_0 {}
impl NoOperandsInstruction for FCONST_1 {}
impl NoOperandsInstruction for FCONST_2 {}
impl NoOperandsInstruction for ICONST_M1 {}
impl NoOperandsInstruction for ICONST_0 {}
impl NoOperandsInstruction for ICONST_1 {}
impl NoOperandsInstruction for ICONST_2 {}
impl NoOperandsInstruction for ICONST_3 {}
impl NoOperandsInstruction for ICONST_4 {}
impl NoOperandsInstruction for ICONST_5 {}
impl NoOperandsInstruction for LCONST_0 {}
impl NoOperandsInstruction for LCONST_1 {}

impl Instruction for ACONST_NULL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_ref(None);
    }
}
impl Instruction for DCONST_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_double(0.0);
    }
}
impl Instruction for DCONST_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(-1);
    }
}
impl Instruction for FCONST_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_float(0.0);
    }
}
impl Instruction for FCONST_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_float(1.0);
    }
}
impl Instruction for FCONST_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_float(2.0);
    }
}
impl Instruction for ICONST_M1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(-1);
    }
}
impl Instruction for ICONST_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(0);
    }
}
impl Instruction for ICONST_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(1);
    }
}
impl Instruction for ICONST_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(2);
    }
}
impl Instruction for ICONST_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(3);
    }
}
impl Instruction for ICONST_4 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(4);
    }
}
impl Instruction for ICONST_5 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_int(5);
    }
}
impl Instruction for LCONST_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_long(0);
    }
}
impl Instruction for LCONST_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        frame.operand_stack.push_long(1);
    }
}
