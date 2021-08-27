use super::class::*;
use crate::runtime::local_vars::*;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Clone)]
pub struct Object<'a> {
  pub class: Weak<RefCell<Class<'a>>>,
  pub field: FieldVar<'a>,
}

impl<'a> Object<'a> {
  pub fn new(class: Weak<RefCell<Class<'a>>>) -> Object<'a> {
    Object {
      class: class.clone(),
      field: FieldVar::new(class.upgrade().unwrap().borrow().instance_slot_count as usize),
    }
  }
  pub fn is_instance_of(&self, class: Weak<RefCell<Class<'a>>>) -> bool {
    class
      .upgrade()
      .unwrap()
      .borrow()
      .is_assignable_from(self.class.clone())
  }
}
