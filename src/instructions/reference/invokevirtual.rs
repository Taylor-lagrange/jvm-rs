use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct INVOKE_VIRTUAL {}

impl Index16Instruction for INVOKE_VIRTUAL {}

impl Instruction for INVOKE_VIRTUAL {
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
    if let ConstantInfoRunTime::Methodref(refs) = info {
      if refs.member_ref.name == "println" {
        match refs.member_ref.descriptor.as_str() {
          "(Z)V" => {
            println!("{}", frame.operand_stack.pop_int() != 0)
          }
          "(C)V" => {
            println!("{}", frame.operand_stack.pop_int())
          }
          "(I)V" | "(B)V" | "(S)V" => {
            println!("{}", frame.operand_stack.pop_int())
          }
          "(F)V" => {
            println!("{}", frame.operand_stack.pop_float())
          }
          "(J)V" => {
            println!("{}", frame.operand_stack.pop_long())
          }
          "(D)V" => {
            println!("{}", frame.operand_stack.pop_double())
          }
          _ => panic!("println: {}", refs.member_ref.descriptor),
        }
        frame.operand_stack.pop_ref();
      }
    }
  }
}
