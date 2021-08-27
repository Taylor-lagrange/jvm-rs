use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct GET_FIELD {}

impl Index16Instruction for GET_FIELD {}

impl Instruction for GET_FIELD {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let info;
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
    }
    if let ConstantInfoRunTime::Fieldref(mut refs) = info {
      let field = refs.resolve_field();
      let rc = field.clone().upgrade().unwrap();
      let field_instance = rc.borrow();
      if field_instance.class_member.is_static() {
        panic!("java.lang.IncompatibleClassChangeError");
      }
      let slot_id = field_instance.slot_id as usize;
      let target_ref = frame.operand_stack.pop_ref();
      if target_ref.is_none() {
        panic!("java.lang.NullPointerException");
      }
      match field_instance
        .class_member
        .descriptor
        .chars()
        .nth(0)
        .unwrap()
      {
        'z' | 'B' | 'C' | 'S' | 'I' => frame
          .operand_stack
          .push_int(target_ref.unwrap().borrow_mut().field.get_int(slot_id)),
        'F' => frame
          .operand_stack
          .push_float(target_ref.unwrap().borrow_mut().field.get_float(slot_id)),
        'J' => frame
          .operand_stack
          .push_long(target_ref.unwrap().borrow_mut().field.get_long(slot_id)),
        'D' => frame
          .operand_stack
          .push_double(target_ref.unwrap().borrow_mut().field.get_double(slot_id)),
        'L' | '[' => frame
          .operand_stack
          .push_ref(target_ref.unwrap().borrow_mut().field.get_ref(slot_id)),
        _ => panic!("todo"),
      }
    }
  }
}
