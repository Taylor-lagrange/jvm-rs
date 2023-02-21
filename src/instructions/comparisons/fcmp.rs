use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

instruction!(FCMPG, NoOperandsInstruction);
instruction!(FCMPL, NoOperandsInstruction);

impl Instruction for FCMPG {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        fcmp(frame, true);
    }
}

impl Instruction for FCMPL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        fcmp(frame, false);
    }
}

fn fcmp(frame: &mut Frame, g_flag: bool) {
    let v2 = frame.operand_stack.pop_float();
    let v1 = frame.operand_stack.pop_float();
    if v1 > v2 {
        frame.operand_stack.push_int(1);
    } else if v1 == v2 {
        frame.operand_stack.push_int(0);
    } else if v1 < v2 {
        frame.operand_stack.push_int(-1);
    } else if g_flag {
        frame.operand_stack.push_int(1);
    } else {
        frame.operand_stack.push_int(-1);
    }
}
