use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::instructions::base::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::method::*;
use crate::runtime::operand_stack::*;
use crate::runtime::thread::*;
use std::rc::Rc;

pub struct INVOKE_INTERFACE {}

impl Index16Instruction for INVOKE_INTERFACE {
  fn fetch_operands(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) -> usize {
    let data = reader.read_u16();
    reader.read_u8(); // count
    reader.read_u8(); // must be 0
    frame.next_pc = reader.pc;
    data as usize
  }
}

impl Instruction for INVOKE_INTERFACE {
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
    if let ConstantInfoRunTime::InterfaceMethodref(mut method_ref) = info {
      let method = method_ref.resolve_interface_method();
      let class = method_ref.member_ref.sym_ref.resolved_class();
      let method_rc = method.upgrade().unwrap();
      {
        let method_instance = method_rc.borrow();

        if method_instance.class_member.is_static() || method_instance.class_member.is_private() {
          panic!("java.lang.IncompatibleClassChangeError");
        }

        let ref_class = frame
          .operand_stack
          .get_ref_from_top(method_instance.arg_slot_count as usize - 1);

        // make sure the class pointer `this` is not null
        if let Slot::RefObject(None) = ref_class {
          panic!("java.lang.NullPointerException");
        }

        let ref_class_instance;

        if let Slot::RefObject(Some(object)) = ref_class {
          ref_class_instance = object.borrow().class.clone();
        } else {
          panic!("ref class error!");
        }

        // if !ref.Class().IsImplements(methodRef.ResolvedClass()) {
        //   panic("java.lang.IncompatibleClassChangeError")
        // }

        let invoke_method = lookup_method_in_class(
          ref_class_instance,
          &method_instance.class_member.name,
          &method_instance.class_member.descriptor,
        );

        if invoke_method.upgrade().is_none() {
          panic!("java.lang.AbstractMethodError");
        }

        let method_to_be_invoke = invoke_method.upgrade().unwrap();
        if method_to_be_invoke.borrow().is_abstract() {
          panic!("java.lang.AbstractMethodError");
        }
        if !method_to_be_invoke.borrow().class_member.is_public() {
          panic!("java.lang.IllegalAccessError");
        }
        method_invoke::invoke_method(frame, method_to_be_invoke);
      }
    }
  }
}
