use super::class::*;
use crate::runtime::operand_stack::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Object<'a> {
  class: Weak<RefCell<Class<'a>>>,
  field: Vec<Slot<'a>>,
}

impl<'a> Object<'a> {
  pub fn new(class: Weak<RefCell<Class<'a>>>) -> Object<'a> {
    Object {
      class: class.clone(),
      field: Vec::with_capacity(class.upgrade().unwrap().borrow().instance_slot_count as usize),
    }
  }
}
