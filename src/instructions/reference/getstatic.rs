use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct GET_STATIC {}

impl Index16Instruction for GET_STATIC {}

impl Instruction for GET_STATIC {
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
      let class = refs.member_ref.sym_ref.resolved_class();
      //TODO:init class
      let rc = field.clone().upgrade().unwrap();
      let field_instance = rc.borrow();
      if !field_instance.class_member.is_static() {
        panic!("java.lang.IncompatibleClassChangeError");
      }
      let field_class = class.upgrade().unwrap(); //class get from const_pool index
      let class_var = &mut field_class.borrow_mut().static_vars;
      let slot_id = field_instance.slot_id as usize;
      match field_instance
        .class_member
        .descriptor
        .chars()
        .nth(0)
        .unwrap()
      {
        'z' | 'B' | 'C' | 'S' | 'I' => frame.operand_stack.push_int(class_var.get_int(slot_id)),
        'F' => frame.operand_stack.push_float(class_var.get_float(slot_id)),
        'J' => frame.operand_stack.push_long(class_var.get_long(slot_id)),
        'D' => frame
          .operand_stack
          .push_double(class_var.get_double(slot_id)),
        'L' | '[' => frame.operand_stack.push_ref(class_var.get_ref(slot_id)),
        _ => panic!("todo"),
      }
    }
  }
}
