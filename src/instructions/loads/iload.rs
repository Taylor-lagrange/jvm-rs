use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ILOAD {}
pub struct ILOAD_0 {}
pub struct ILOAD_1 {}
pub struct ILOAD_2 {}
pub struct ILOAD_3 {}

impl Index8Instruction for ILOAD {}
impl NoOperandsInstruction for ILOAD_0 {}
impl NoOperandsInstruction for ILOAD_1 {}
impl NoOperandsInstruction for ILOAD_2 {}
impl NoOperandsInstruction for ILOAD_3 {}

fn iload(frame: &mut Frame, index: usize) {
    frame
        .operand_stack
        .push_int(frame.local_vars.get_int(index))
}

impl Instruction for ILOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        iload(frame, index);
    }
}

impl Instruction for ILOAD_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        iload(frame, 0);
    }
}

impl Instruction for ILOAD_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        iload(frame, 1);
    }
}

impl Instruction for ILOAD_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        iload(frame, 2);
    }
}

impl Instruction for ILOAD_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        iload(frame, 3);
    }
}
