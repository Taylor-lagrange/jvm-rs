use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

instruction!(IF_ACMPEQ, BranchInstruction);
instruction!(IF_ACMPNE, BranchInstruction);

impl Instruction for IF_ACMPEQ {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_ref();
        let v1 = frame.operand_stack.pop_ref();
        if v1.is_none() && v2.is_none() {
            branch(frame, offset);
        }
        // 该指令也会用于两个 String 的比较，因为 java 里的 String 不可变，且有 String intern pool
        // 可以认为只要两个 String 指向的地址相同，两个 String 就是一样的
        if (v1.is_some() && v2.is_some()) && std::ptr::eq(&v1.unwrap(), &v2.unwrap()) {
            branch(frame, offset);
        }
    }
}

impl Instruction for IF_ACMPNE {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let offset = self.fetch_operands(reader, frame);
        let v2 = frame.operand_stack.pop_ref();
        let v1 = frame.operand_stack.pop_ref();
        if !(v1.is_none() && v2.is_none())
            && !((v1.is_some() && v2.is_some()) && std::ptr::eq(&v1.unwrap(), &v2.unwrap()))
        {
            branch(frame, offset);
        }
    }
}
