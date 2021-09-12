use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct ASTORE {}
pub struct ASTORE_0 {}
pub struct ASTORE_1 {}
pub struct ASTORE_2 {}
pub struct ASTORE_3 {}

impl Index8Instruction for ASTORE {}
impl NoOperandsInstruction for ASTORE_0 {}
impl NoOperandsInstruction for ASTORE_1 {}
impl NoOperandsInstruction for ASTORE_2 {}
impl NoOperandsInstruction for ASTORE_3 {}

fn astore(frame: &mut Frame, index: usize) {
    let ref_object = frame.operand_stack.pop_ref();
    frame.local_vars.set_ref(index, ref_object);
}

impl Instruction for ASTORE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        astore(frame, index);
    }
}

impl Instruction for ASTORE_0 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        astore(frame, 0);
    }
}

impl Instruction for ASTORE_1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        astore(frame, 1);
    }
}

impl Instruction for ASTORE_2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        astore(frame, 2);
    }
}

impl Instruction for ASTORE_3 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        astore(frame, 3);
    }
}
