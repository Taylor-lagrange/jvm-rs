use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DLOAD {}
pub struct DLOAD_0 {}
pub struct DLOAD_1 {}
pub struct DLOAD_2 {}
pub struct DLOAD_3 {}

impl Index8Instruction for DLOAD {}
impl NoOperandsInstruction for DLOAD_0 {}
impl NoOperandsInstruction for DLOAD_1 {}
impl NoOperandsInstruction for DLOAD_2 {}
impl NoOperandsInstruction for DLOAD_3 {}

fn dload(frame: &mut Frame, index: usize) {
    frame
        .operand_stack
        .push_double(frame.local_vars.get_double(index))
}

impl Instruction for DLOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        dload(frame, index);
    }
}

impl Instruction for DLOAD_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        dload(frame, 0);
    }
}

impl Instruction for DLOAD_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        dload(frame, 1);
    }
}

impl Instruction for DLOAD_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        dload(frame, 2);
    }
}

impl Instruction for DLOAD_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        dload(frame, 3);
    }
}
