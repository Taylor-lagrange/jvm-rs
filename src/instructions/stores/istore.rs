use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ISTORE {}
pub struct ISTORE_0 {}
pub struct ISTORE_1 {}
pub struct ISTORE_2 {}
pub struct ISTORE_3 {}

impl Index8Instruction for ISTORE {}
impl NoOperandsInstruction for ISTORE_0 {}
impl NoOperandsInstruction for ISTORE_1 {}
impl NoOperandsInstruction for ISTORE_2 {}
impl NoOperandsInstruction for ISTORE_3 {}

fn istore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_int();
    frame.local_vars.set_int(index, val);
}

impl Instruction for ISTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        istore(frame, index);
    }
}

impl Instruction for ISTORE_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        istore(frame, 0);
    }
}

impl Instruction for ISTORE_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        istore(frame, 1);
    }
}

impl Instruction for ISTORE_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        istore(frame, 2);
    }
}

impl Instruction for ISTORE_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        istore(frame, 3);
    }
}
