use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

pub struct DUP {}
pub struct DUP_X1 {}
pub struct DUP_X2 {}
pub struct DUP2 {}
pub struct DUP2_X1 {}
pub struct DUP2_X2 {}

impl NoOperandsInstruction for DUP {}
impl NoOperandsInstruction for DUP_X1 {}
impl NoOperandsInstruction for DUP_X2 {}
impl NoOperandsInstruction for DUP2 {}
impl NoOperandsInstruction for DUP2_X1 {}
impl NoOperandsInstruction for DUP2_X2 {}

/*
DUP: Duplicate the top operand stack value

bottom -> top
[...][c][b][a]
             \_
               |
               V
[...][c][b][a][a]
*/
impl Instruction for DUP {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let slot = frame.operand_stack.pop_slot();
        frame.operand_stack.push_slot(slot.clone());
        frame.operand_stack.push_slot(slot);
    }
}

/*
DUP_X1: Duplicate the top operand stack value and insert two values down

bottom -> top
[...][c][b][a]
          __/
         |
         V
[...][c][a][b][a]
*/
impl Instruction for DUP_X1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_slot();
        let slot2 = frame.operand_stack.pop_slot();
        frame.operand_stack.push_slot(slot1.clone());
        frame.operand_stack.push_slot(slot2);
        frame.operand_stack.push_slot(slot1);
    }
}

/*
DUP_X2: Duplicate the top operand stack value and insert two or three values down

bottom -> top
[...][c][b][a]
       _____/
      |
      V
[...][a][c][b][a]
*/
impl Instruction for DUP_X2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_slot();
        let slot2 = frame.operand_stack.pop_slot();
        let slot3 = frame.operand_stack.pop_slot();
        frame.operand_stack.push_slot(slot1.clone());
        frame.operand_stack.push_slot(slot3);
        frame.operand_stack.push_slot(slot2);
        frame.operand_stack.push_slot(slot1);
    }
}

/*
DUP2: Duplicate the top one or two operand stack values

bottom -> top
[...][c][b][a]____
          \____   |
               |  |
               V  V
[...][c][b][a][b][a]
*/
impl Instruction for DUP2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_slot();
        let slot2 = frame.operand_stack.pop_slot();
        frame.operand_stack.push_slot(slot2.clone());
        frame.operand_stack.push_slot(slot1.clone());
        frame.operand_stack.push_slot(slot2);
        frame.operand_stack.push_slot(slot1);
    }
}

/*
DUP2_X1: Duplicate the top one or two operand stack values and insert two or three values down

bottom -> top
[...][c][b][a]
       _/ __/
      |  |
      V  V
[...][b][a][c][b][a]
*/
impl Instruction for DUP2_X1 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_slot();
        let slot2 = frame.operand_stack.pop_slot();
        let slot3 = frame.operand_stack.pop_slot();
        frame.operand_stack.push_slot(slot2.clone());
        frame.operand_stack.push_slot(slot1.clone());
        frame.operand_stack.push_slot(slot3);
        frame.operand_stack.push_slot(slot2);
        frame.operand_stack.push_slot(slot1);
    }
}

/*
DUP2_X2: Duplicate the top one or two operand stack values and insert two, three, or four values down

bottom -> top
[...][d][c][b][a]
       ____/ __/
      |   __/
      V  V
[...][b][a][d][c][b][a]
*/
impl Instruction for DUP2_X2 {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_slot();
        let slot2 = frame.operand_stack.pop_slot();
        let slot3 = frame.operand_stack.pop_slot();
        let slot4 = frame.operand_stack.pop_slot();
        frame.operand_stack.push_slot(slot2.clone());
        frame.operand_stack.push_slot(slot1.clone());
        frame.operand_stack.push_slot(slot4);
        frame.operand_stack.push_slot(slot3);
        frame.operand_stack.push_slot(slot2);
        frame.operand_stack.push_slot(slot1);
    }
}
