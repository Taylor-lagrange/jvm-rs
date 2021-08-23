use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct NEW {}

impl Index16Instruction for NEW {}

impl Instruction for NEW {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
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
    if let ConstantInfoRunTime::Class(refs) = cp.get_constant_info(index) {
      let class = refs.sym_ref.resolved_class();
      let rc = class.clone().upgrade().unwrap();
      let class_instance = rc.borrow();
      if class_instance.is_interface() || class_instance.is_abstract() {
        panic!("java.lang.InstantiationError");
      }
      let ref_obj = Class::new_object(&class.upgrade().unwrap());
      frame.operand_stack.push_ref(Some(ref_obj));
    }
  }
}
