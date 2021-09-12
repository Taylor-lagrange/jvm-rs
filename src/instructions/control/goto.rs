use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct GOTO {}

impl BranchInstruction for GOTO {}

impl Instruction for GOTO {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        branch(frame, offset as i32);
    }
}
