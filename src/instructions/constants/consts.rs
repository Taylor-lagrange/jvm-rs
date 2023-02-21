use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

instruction!(ACONST_NULL, NoOperandsInstruction);
instruction!(DCONST_0, NoOperandsInstruction);
instruction!(DCONST_1, NoOperandsInstruction);
instruction!(FCONST_0, NoOperandsInstruction);
instruction!(FCONST_1, NoOperandsInstruction);
instruction!(FCONST_2, NoOperandsInstruction);
instruction!(ICONST_M1, NoOperandsInstruction);
instruction!(ICONST_0, NoOperandsInstruction);
instruction!(ICONST_1, NoOperandsInstruction);
instruction!(ICONST_2, NoOperandsInstruction);
instruction!(ICONST_3, NoOperandsInstruction);
instruction!(ICONST_4, NoOperandsInstruction);
instruction!(ICONST_5, NoOperandsInstruction);
instruction!(LCONST_0, NoOperandsInstruction);
instruction!(LCONST_1, NoOperandsInstruction);

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
