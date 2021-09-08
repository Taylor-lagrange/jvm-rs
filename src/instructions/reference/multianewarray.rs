use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::object::*;
use crate::runtime::operand_stack::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct MULTI_ANEW_ARRAY {}

impl Instruction for MULTI_ANEW_ARRAY {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = reader.read_u16();
    let dimentions = reader.read_u8();
    frame.next_pc = reader.pc;
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
      info = cp.get_constant_info(index as usize).clone();
    }
    if let ConstantInfoRunTime::Class(mut class_ref) = info {
      let arr_class = class_ref.sym_ref.resolved_class();
      let counts = pop_and_check_counts(&mut frame.operand_stack, dimentions);
      let arr = new_multi_dimensional_array(0, &counts, arr_class);
      frame
        .operand_stack
        .push_ref(Some(Rc::new(RefCell::new(arr))));
    }
  }
}

fn new_multi_dimensional_array<'a>(
  deep: usize,
  counts: &Vec<usize>,
  arr_class: Weak<RefCell<Class<'a>>>,
) -> Object<'a> {
  let count = counts[deep];
  let mut arr = Object::new_array(arr_class.clone(), count);
  if deep < counts.len() {
    if let ObjectData::ArrayRefs(refs) = &mut arr.data {
      for i in 0..refs.len() {
        let obj =
          new_multi_dimensional_array(deep + 1, counts, Class::component_class(arr_class.clone()));
        refs[i] = Rc::new(RefCell::new(obj));
      }
    }
  }
  arr
}

fn pop_and_check_counts(stack: &mut OperandStack, dimentions: u8) -> Vec<usize> {
  let mut v = Vec::with_capacity(dimentions as usize);
  for _ in 0..dimentions {
    v.push(0);
  }
  for i in (0..dimentions).rev() {
    v[i as usize] = stack.pop_int() as usize;
    if v[i as usize] < 0 {
      panic!("java.lang.NegativeArraySizeException");
    }
  }
  v
}
