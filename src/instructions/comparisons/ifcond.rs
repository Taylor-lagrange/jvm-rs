use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct IFEQ {}
pub struct IFNE {}
pub struct IFLT {}
pub struct IFLE {}
pub struct IFGT {}
pub struct IFGE {}

impl BranchInstruction for IFEQ {}
impl BranchInstruction for IFNE {}
impl BranchInstruction for IFLT {}
impl BranchInstruction for IFLE {}
impl BranchInstruction for IFGT {}
impl BranchInstruction for IFGE {}

impl Instruction for IFEQ {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_int();
        if v == 0 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IFNE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_int();
        if v != 0 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IFLT {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_int();
        if v < 0 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IFLE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_int();
        if v <= 0 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IFGT {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_int();
        if v > 0 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IFGE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v = frame.operand_stack.pop_int();
        if v >= 0 {
            branch(frame, offset);
        }
    }
}
