use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::instructions::base::class_init::*;
use crate::runtime::heap::class::*;
use crate::runtime::thread::*;
use std::rc::Rc;

pub struct PUT_STATIC {}

impl Index16Instruction for PUT_STATIC {}

impl Instruction for PUT_STATIC {
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
      if !Class::init_started(&class) {
        frame.revert_pc();
        init_class(frame.thread.clone(), class.clone());
        return;
      }
      let rc = field.clone().upgrade().unwrap();
      let field_instance = rc.borrow();
      if !field_instance.class_member.is_static() {
        panic!("java.lang.IncompatibleClassChangeError");
      }
      let field_class = class.upgrade().unwrap(); //class get from const_pool index
      if field_instance.class_member.is_final() {
        // 如果是final字段，则实际操作的是静态常量，只能在类初始化方法中给它赋值。
        // 否则，会抛出IllegalAccessError异常。类初始化⽅法由编译器⽣成，名字是<clinit>
        if !Rc::ptr_eq(&pool_class, &field_class)
          || frame.method.borrow().class_member.name != "<clinit>"
        {
          panic!("java.lang.IllegalAccessError");
        }
      }
      let class_var = &mut field_class.borrow_mut().static_vars;
      let slot_id = field_instance.slot_id as usize;
      match field_instance
        .class_member
        .descriptor
        .chars()
        .nth(0)
        .unwrap()
      {
        'z' | 'B' | 'C' | 'S' | 'I' => class_var.set_int(slot_id, frame.operand_stack.pop_int()),
        'F' => class_var.set_float(slot_id, frame.operand_stack.pop_float()),
        'J' => class_var.set_long(slot_id, frame.operand_stack.pop_long()),
        'D' => class_var.set_double(slot_id, frame.operand_stack.pop_double()),
        'L' | '[' => class_var.set_ref(slot_id, frame.operand_stack.pop_ref()),
        _ => panic!("todo"),
      }
    }
  }
}
