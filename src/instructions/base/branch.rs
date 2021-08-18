use super::bytecode_reader::*;
use crate::runtime::thread::*;

pub fn branch(frame: &mut Frame, offset: i32) {
  frame.next_pc = (frame.thread.borrow_mut().pc as i32 + offset) as usize;
}
