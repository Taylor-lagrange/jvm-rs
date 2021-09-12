use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct RETURN {}
pub struct ARETURN {}
pub struct DRETURN {}
pub struct FRETURN {}
pub struct IRETURN {}
pub struct LRETURN {}

impl NoOperandsInstruction for RETURN {}
impl NoOperandsInstruction for ARETURN {}
impl NoOperandsInstruction for DRETURN {}
impl NoOperandsInstruction for FRETURN {}
impl NoOperandsInstruction for IRETURN {}
impl NoOperandsInstruction for LRETURN {}

impl Instruction for RETURN {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let rc = frame.thread.clone().upgrade().unwrap();
        let mut thread = rc.borrow_mut();
        thread.stack.pop();
    }
}

impl Instruction for ARETURN {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let rc = frame.thread.clone().upgrade().unwrap();
        let mut thread = rc.borrow_mut();
        // pop the current frame
        thread.stack.pop();
        let invoker = thread.stack.top();
        let val = frame.operand_stack.pop_ref();
        invoker.borrow_mut().operand_stack.push_ref(val);
    }
}
impl Instruction for DRETURN {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let rc = frame.thread.clone().upgrade().unwrap();
        let mut thread = rc.borrow_mut();
        thread.stack.pop();
        let invoker = thread.stack.top();
        let val = frame.operand_stack.pop_double();
        invoker.borrow_mut().operand_stack.push_double(val);
    }
}
impl Instruction for FRETURN {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let rc = frame.thread.clone().upgrade().unwrap();
        let mut thread = rc.borrow_mut();
        thread.stack.pop();
        let invoker = thread.stack.top();
        let val = frame.operand_stack.pop_float();
        invoker.borrow_mut().operand_stack.push_float(val);
    }
}
impl Instruction for IRETURN {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let rc = frame.thread.clone().upgrade().unwrap();
        let mut thread = rc.borrow_mut();
        thread.stack.pop();
        let invoker = thread.stack.top();
        let val = frame.operand_stack.pop_int();
        invoker.borrow_mut().operand_stack.push_int(val);
    }
}
impl Instruction for LRETURN {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let rc = frame.thread.clone().upgrade().unwrap();
        let mut thread = rc.borrow_mut();
        thread.stack.pop();
        let invoker = thread.stack.top();
        let val = frame.operand_stack.pop_long();
        invoker.borrow_mut().operand_stack.push_long(val);
    }
}
