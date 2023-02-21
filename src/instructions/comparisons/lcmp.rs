use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

instruction!(LCMP, NoOperandsInstruction);

impl Instruction for LCMP {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let v2 = frame.operand_stack.pop_long();
        let v1 = frame.operand_stack.pop_long();
        if v1 > v2 {
            frame.operand_stack.push_int(1);
        } else if v1 == v2 {
            frame.operand_stack.push_int(0);
        } else {
            frame.operand_stack.push_int(-1);
        }
    }
}
