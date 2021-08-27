use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct INSTANCE_OF {}

impl Index16Instruction for INSTANCE_OF {}

impl Instruction for INSTANCE_OF {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    let target_ref = frame.operand_stack.pop_ref();
    if target_ref.is_none() {
      frame.operand_stack.push_int(0);
      return;
    }
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
    if let ConstantInfoRunTime::Class(mut refs) = info{
      let class = refs.sym_ref.resolved_class();
      if target_ref.unwrap().borrow().is_instance_of(class) {
        frame.operand_stack.push_int(1);
      } else {
        frame.operand_stack.push_int(0);
      }
    }
  }
}
