use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct IFNULL {}
pub struct IFNONNULL {}

impl BranchInstruction for IFNULL {}
impl BranchInstruction for IFNONNULL {}

impl Instruction for IFNULL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_ref();
        if v.is_none() {
            branch(frame, offset as i32);
        }
    }
}

impl Instruction for IFNONNULL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_ref();
        if !v.is_none() {
            branch(frame, offset as i32);
        }
    }
}
