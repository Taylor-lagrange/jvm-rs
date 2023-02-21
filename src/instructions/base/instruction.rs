use super::bytecode_reader::*;
use crate::runtime::thread::*;

pub trait Instruction {
    // fn fetch_operands(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) -> usize;
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame);
}

pub trait NoOperandsInstruction: Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) -> usize {
        0
    }
}

pub trait BranchInstruction: Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) -> i32 {
        let data = reader.read_i16();
        frame.next_pc = reader.pc;
        data as i32
    }
}

pub trait Index8Instruction: Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) -> usize {
        let data = reader.read_u8();
        frame.next_pc = reader.pc;
        data as usize
    }
}

pub trait Index16Instruction: Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) -> usize {
        let data = reader.read_u16();
        frame.next_pc = reader.pc;
        data as usize
    }
}

macro_rules! instruction {
    ($instruction_name:ident, $operand_type:ident) => {
        pub struct $instruction_name {}
        impl $operand_type for $instruction_name {}
    };
}

pub(crate) use instruction;
