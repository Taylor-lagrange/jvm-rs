use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

instruction!(IF_ICMPEQ, BranchInstruction);
instruction!(IF_ICMPNE, BranchInstruction);
instruction!(IF_ICMPLT, BranchInstruction);
instruction!(IF_ICMPLE, BranchInstruction);
instruction!(IF_ICMPGT, BranchInstruction);
instruction!(IF_ICMPGE, BranchInstruction);

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
