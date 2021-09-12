use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct GOTO_W {}

impl BranchInstruction for GOTO_W {}

impl Instruction for GOTO_W {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = reader.read_i32();
        frame.next_pc = reader.pc;
        branch(frame, offset);
    }
}
