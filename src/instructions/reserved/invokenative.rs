use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::native::registry::*;
use crate::runtime::thread::*;
use std::sync::Arc;

// Invoke native method
pub struct INVOKE_NATIVE {}

impl NoOperandsInstruction for INVOKE_NATIVE {}

impl Instruction for INVOKE_NATIVE {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let native_method;
    let key;
    {
      let method = frame.method.borrow();
      let temp = method.class_member.class.clone().upgrade().unwrap();
      let rc = temp.borrow();
      let class_name = &rc.name;
      let method_name = &method.class_member.name;
      let method_descriptor = &method.class_member.descriptor;
      key = format!("{}~{}~{}", class_name, method_name, method_descriptor);
      native_method = find_native_method(class_name, method_name, method_descriptor);
    }
    if Arc::ptr_eq(&native_method, &INVALID_NATIVE_METHOD) {
      panic!("java.lang.UnsatisfiedLinkError: {}", key);
    }
    native_method(frame);
  }
}
