use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ANEW_ARRAY {}

impl Index16Instruction for ANEW_ARRAY {}

impl Instruction for ANEW_ARRAY {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    let info;
    {
      let rc = frame.method.borrow_mut().class_member.class.clone();
      let pool_rc = rc
        .upgrade()
        .unwrap()
        .borrow_mut()
        .constant_pool
        .clone()
        .unwrap();
      let mut cp = pool_rc.borrow_mut();
      info = cp.get_constant_info(index).clone();
    }
    if let ConstantInfoRunTime::Class(mut class_ref) = info {
      let component_class = class_ref.sym_ref.resolved_class();
      let count = frame.operand_stack.pop_int();
      if count < 0 {
        panic!("java.lang.NegativeArraySizeException");
      }
      let arr_class = Class::get_array_class(component_class);
      let obj = Object::new_array(arr_class, count as usize);
      frame
        .operand_stack
        .push_ref(Some(Rc::new(RefCell::new(obj))));
    }
  }
}
