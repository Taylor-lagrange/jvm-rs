use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct Nop {}

impl NoOperandsInstruction for Nop {}

impl Instruction for Nop {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {}
}
