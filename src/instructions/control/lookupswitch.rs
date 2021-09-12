use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;

/*
lookupswitch
<0-3 byte pad>
defaultbyte1
defaultbyte2
defaultbyte3
defaultbyte4
npairs1
npairs2
npairs3
npairs4
match-offset pairs...
*/

// Access jump table by key match and jump
struct LookTable {
    default_offset: i32,
    npairs: i32,
    match_offsets: Vec<i32>,
}

pub struct LOOKUP_SWITCH {}

impl BranchInstruction for LOOKUP_SWITCH {}

impl Instruction for LOOKUP_SWITCH {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        reader.skip_padding();
        let mut table = LookTable {
            default_offset: reader.read_i32(),
            npairs: reader.read_i32(),
            match_offsets: Vec::new(),
        };
        table.match_offsets = reader.read_i32s((table.npairs * 2) as usize);
        frame.next_pc = reader.pc;

        let key = frame.operand_stack.pop_int();
        let mut i: usize = 0;
        while i < (table.npairs * 2) as usize {
            if table.match_offsets[i] == key {
                branch(frame, table.match_offsets[i + 1] as i32);
            }
            i += 2
        }
        branch(frame, table.default_offset as i32);
    }
}
