use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct IF_ICMPEQ {}
pub struct IF_ICMPNE {}
pub struct IF_ICMPLT {}
pub struct IF_ICMPLE {}
pub struct IF_ICMPGT {}
pub struct IF_ICMPGE {}

impl BranchInstruction for IF_ICMPEQ {}
impl BranchInstruction for IF_ICMPNE {}
impl BranchInstruction for IF_ICMPLT {}
impl BranchInstruction for IF_ICMPLE {}
impl BranchInstruction for IF_ICMPGT {}
impl BranchInstruction for IF_ICMPGE {}

impl Instruction for IF_ICMPEQ {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v1 == v2 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IF_ICMPNE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v1 != v2 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IF_ICMPLT {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v1 < v2 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IF_ICMPLE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v1 <= v2 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IF_ICMPGT {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v1 > v2 {
            branch(frame, offset);
        }
    }
}

impl Instruction for IF_ICMPGE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_int();
        let v1 = frame.operand_stack.pop_int();
        if v1 >= v2 {
            branch(frame, offset);
        }
    }
}
