use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct LSTORE {}
pub struct LSTORE_0 {}
pub struct LSTORE_1 {}
pub struct LSTORE_2 {}
pub struct LSTORE_3 {}

impl Index8Instruction for LSTORE {}
impl NoOperandsInstruction for LSTORE_0 {}
impl NoOperandsInstruction for LSTORE_1 {}
impl NoOperandsInstruction for LSTORE_2 {}
impl NoOperandsInstruction for LSTORE_3 {}

fn lstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_long();
    frame.local_vars.set_long(index, val);
}

impl Instruction for LSTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        lstore(frame, index);
    }
}

impl Instruction for LSTORE_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        lstore(frame, 0);
    }
}

impl Instruction for LSTORE_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        lstore(frame, 1);
    }
}

impl Instruction for LSTORE_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        lstore(frame, 2);
    }
}

impl Instruction for LSTORE_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        lstore(frame, 3);
    }
}
