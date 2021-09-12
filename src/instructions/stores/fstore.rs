use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct FSTORE {}
pub struct FSTORE_0 {}
pub struct FSTORE_1 {}
pub struct FSTORE_2 {}
pub struct FSTORE_3 {}

impl Index8Instruction for FSTORE {}
impl NoOperandsInstruction for FSTORE_0 {}
impl NoOperandsInstruction for FSTORE_1 {}
impl NoOperandsInstruction for FSTORE_2 {}
impl NoOperandsInstruction for FSTORE_3 {}

fn fstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_float();
    frame.local_vars.set_float(index, val);
}

impl Instruction for FSTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        fstore(frame, index);
    }
}

impl Instruction for FSTORE_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        fstore(frame, 0);
    }
}

impl Instruction for FSTORE_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        fstore(frame, 1);
    }
}

impl Instruction for FSTORE_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        fstore(frame, 2);
    }
}

impl Instruction for FSTORE_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        fstore(frame, 3);
    }
}
