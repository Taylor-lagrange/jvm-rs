use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct SWAP {}

impl NoOperandsInstruction for SWAP {}

// swap 指令交换栈顶的两个变量
impl Instruction for SWAP {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let slot1 = frame.operand_stack.pop_slot();
    let slot2 = frame.operand_stack.pop_slot();
    frame.operand_stack.push_slot(slot1);
    frame.operand_stack.push_slot(slot2);
  }
}
