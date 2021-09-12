use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

/*
tableswitch
<0-3 byte pad>
defaultbyte1
defaultbyte2
defaultbyte3
defaultbyte4
lowbyte1
lowbyte2
lowbyte3
lowbyte4
highbyte1
highbyte2
highbyte3
highbyte4
jump offsets...
*/

struct JumpTable {
    default_offset: i32,
    low: i32,
    high: i32,
    jump_offsets: Vec<i32>,
}

pub struct TABLE_SWITCH {}

impl BranchInstruction for TABLE_SWITCH {}

impl Instruction for TABLE_SWITCH {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        reader.skip_padding();
        let mut table = JumpTable {
            default_offset: reader.read_i32(),
            low: reader.read_i32(),
            high: reader.read_i32(),
            jump_offsets: Vec::new(),
        };
        table.jump_offsets = reader.read_i32s((table.high - table.low + 1) as usize);
        frame.next_pc = reader.pc;

        let index = frame.operand_stack.pop_int();
        let mut offset = 0;
        if table.low <= index && index <= table.high {
            offset = table.jump_offsets[(index - table.low) as usize];
        } else {
            offset = table.default_offset;
        }
        branch(frame, offset as i32);
    }
}
