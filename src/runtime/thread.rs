use super::local_vars::*;
use super::operand_stack::*;
use crate::utils::list::*;

struct Frame<'a> {
  local_vars: LocalVars<'a>,
  oper_and_stack: OperandStack<'a>,
}

impl<'a> Frame<'a> {
  pub fn new(max_locals: usize, max_stack: usize) -> Frame<'a> {
    Frame {
      local_vars: LocalVars::new(max_locals),
      oper_and_stack: OperandStack::new(max_stack),
    }
  }
}

struct Stack<'a> {
  pub max_size: usize,
  pub frame_list: List<Frame<'a>>,
}

impl<'a> Stack<'a> {
  pub fn new(max_size: usize) -> Stack<'a> {
    Stack {
      max_size: max_size,
      frame_list: List::new(),
    }
  }
  pub fn push(&mut self, frame: Frame<'a>) {
    if self.frame_list.size >= self.max_size as i32 {
      panic!("java.lang.StackOverflowError");
    }
    self.frame_list.push(frame)
  }
  pub fn pop(&mut self) -> Frame<'a> {
    let data = self.frame_list.pop();
    match data {
      Some(frame) => frame,
      None => panic!("jvm stack is empty!"),
    }
  }
  pub fn top(&mut self) -> &mut Frame<'a> {
    let data = self.frame_list.peek_mut();
    match data {
      Some(frame) => frame,
      None => panic!("jvm stack is empty!"),
    }
  }
}

struct Thread<'a> {
  pc: i32,
  stack: Stack<'a>,
}

impl<'a> Thread<'a> {
  pub fn new() -> Thread<'a> {
    Thread {
      pc: 0,
      stack: Stack::new(1024),
    }
  }
}
