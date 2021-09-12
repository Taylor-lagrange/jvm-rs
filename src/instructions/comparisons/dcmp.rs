use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DCMPG {}
pub struct DCMPL {}

impl NoOperandsInstruction for DCMPG {}
impl NoOperandsInstruction for DCMPL {}

impl Instruction for DCMPG {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        dcmp(frame, true);
    }
}

impl Instruction for DCMPL {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        dcmp(frame, false);
    }
}

fn dcmp(frame: &mut Frame, g_flag: bool) {
    let v2 = frame.operand_stack.pop_double();
    let v1 = frame.operand_stack.pop_double();
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
