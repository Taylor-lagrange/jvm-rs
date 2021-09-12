use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ALOAD {}
pub struct ALOAD_0 {}
pub struct ALOAD_1 {}
pub struct ALOAD_2 {}
pub struct ALOAD_3 {}

impl Index8Instruction for ALOAD {}
impl NoOperandsInstruction for ALOAD_0 {}
impl NoOperandsInstruction for ALOAD_1 {}
impl NoOperandsInstruction for ALOAD_2 {}
impl NoOperandsInstruction for ALOAD_3 {}

fn aload(frame: &mut Frame, index: usize) {
    frame
        .operand_stack
        .push_ref(frame.local_vars.get_ref(index))
}

impl Instruction for ALOAD {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        aload(frame, index);
    }
}

impl Instruction for ALOAD_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        aload(frame, 0);
    }
}

impl Instruction for ALOAD_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        aload(frame, 1);
    }
}

impl Instruction for ALOAD_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        aload(frame, 2);
    }
}

impl Instruction for ALOAD_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        aload(frame, 3);
    }
}
