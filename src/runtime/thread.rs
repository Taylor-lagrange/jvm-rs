use super::heap::method::*;
use super::local_vars::*;
use super::operand_stack::*;
use crate::utils::list::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

/*
JVM
  Thread
    pc
    Stack
      Frame
        LocalVars
        OperandStack
*/

pub struct Frame<'a> {
  pub local_vars: LocalVars<'a>,
  pub operand_stack: OperandStack<'a>,
  pub thread: Weak<RefCell<Thread<'a>>>,
  pub method: Rc<RefCell<Method<'a>>>,
  pub next_pc: usize,
}

impl<'a> Frame<'a> {
  pub fn new(method: Rc<RefCell<Method<'a>>>, thread: Weak<RefCell<Thread<'a>>>) -> Frame<'a> {
    let max_locals = method.borrow().max_locals;
    let max_stack = method.borrow().max_stack;
    Frame {
      local_vars: LocalVars::new(max_locals as usize),
      thread: thread,
      method: method,
      operand_stack: OperandStack::new(max_stack as usize),
      next_pc: 0,
    }
  }
}

pub struct Stack<'a> {
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

pub struct Thread<'a> {
  pub pc: i32,
  pub stack: Stack<'a>,
}

impl<'a> Thread<'a> {
  pub fn new() -> Rc<RefCell<Thread<'a>>> {
    Rc::new(RefCell::new(Thread {
      pc: 0,
      stack: Stack::new(1024),
    }))
  }
  pub fn new_frame(thread: Weak<RefCell<Thread<'a>>>, method: Rc<RefCell<Method<'a>>>) -> Frame<'a> {
    Frame::new(method, thread)
  }
}
