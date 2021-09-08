use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;
use std::rc::Rc;

pub struct PUT_FIELD {}

impl Index16Instruction for PUT_FIELD {}

impl Instruction for PUT_FIELD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let info;
    let pool_class;
    {
      let index = self.fetch_operands(reader, frame);
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
      pool_class = cp.class.clone().upgrade().unwrap(); //class get from const_pool pointer
    }
    if let ConstantInfoRunTime::Fieldref(mut refs) = info {
      let field = refs.resolve_field();
      let class = refs.member_ref.sym_ref.resolved_class();
      let rc = field.clone().upgrade().unwrap();
      let field_instance = rc.borrow();
      if field_instance.class_member.is_static() {
        panic!("java.lang.IncompatibleClassChangeError");
      }
      let field_class = class.upgrade().unwrap(); //class get from const_pool index
      if field_instance.class_member.is_final() {
        // 如果是final字段，则实际操作的是静态常量，只能在类初始化方法中给它赋值。
        // 否则，会抛出IllegalAccessError异常。
        if !Rc::ptr_eq(&pool_class, &field_class)
          || frame.method.borrow().class_member.name != "<init>"
        {
          panic!("java.lang.IllegalAccessError");
        }
      }
      let slot_id = field_instance.slot_id as usize;
      match field_instance
        .class_member
        .descriptor
        .chars()
        .nth(0)
        .unwrap()
      {
        'z' | 'B' | 'C' | 'S' | 'I' => {
          let val = frame.operand_stack.pop_int();
          let re = frame.operand_stack.pop_ref();
          if re.is_none() {
            panic!("java.lang.NullPointerException");
          }
          if let ObjectData::Field(field) = &mut re.unwrap().borrow_mut().data {
            field.set_int(slot_id, val);
          }
          // re.unwrap().borrow_mut().data.field.set_int(slot_id, val);
        }
        'F' => {
          let val = frame.operand_stack.pop_float();
          let re = frame.operand_stack.pop_ref();
          if re.is_none() {
            panic!("java.lang.NullPointerException");
          }
          if let ObjectData::Field(field) = &mut re.unwrap().borrow_mut().data {
            field.set_float(slot_id, val);
          }
          // re.unwrap().borrow_mut().data.field.set_float(slot_id, val);
        }
        'J' => {
          let val = frame.operand_stack.pop_long();
          let re = frame.operand_stack.pop_ref();
          if re.is_none() {
            panic!("java.lang.NullPointerException");
          }
          if let ObjectData::Field(field) = &mut re.unwrap().borrow_mut().data {
            field.set_long(slot_id, val);
          }
          // re.unwrap().borrow_mut().data.field.set_long(slot_id, val);
        }
        'D' => {
          let val = frame.operand_stack.pop_double();
          let re = frame.operand_stack.pop_ref();
          if re.is_none() {
            panic!("java.lang.NullPointerException");
          }
          if let ObjectData::Field(field) = &mut re.unwrap().borrow_mut().data {
            field.set_double(slot_id, val);
          }
          // re.unwrap().borrow_mut().data.field.set_double(slot_id, val);
        }
        'L' | '[' => {
          let val = frame.operand_stack.pop_ref();
          let re = frame.operand_stack.pop_ref();
          if re.is_none() {
            panic!("java.lang.NullPointerException");
          }
          if let ObjectData::Field(field) = &mut re.unwrap().borrow_mut().data {
            field.set_ref(slot_id, val);
          }
          // re.unwrap().borrow_mut().data.field.set_ref(slot_id, val);
        }
        _ => panic!("todo"),
      }
    }
  }
}
