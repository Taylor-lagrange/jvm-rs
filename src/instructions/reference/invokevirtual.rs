use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::instructions::base::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::method::*;
use crate::runtime::heap::string_pool::*;
use crate::runtime::operand_stack::*;
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
    if let ConstantInfoRunTime::Methodref(mut method_ref) = info {
      let method = method_ref.resolve_method();
      let class = method_ref.member_ref.sym_ref.resolved_class();
      let method_rc = method.upgrade().unwrap();
      let class_rc = class.upgrade().unwrap();
      {
        let method_instance = method_rc.borrow();
        let class_instance = class_rc.borrow();

        if method_instance.class_member.is_static() {
          panic!("java.lang.IncompatibleClassChangeError");
        }

        let ref_class = frame
          .operand_stack
          .get_ref_from_top(method_instance.arg_slot_count as usize - 1);

        // make sure the class pointer `this` is not null
        if let Slot::RefObject(None) = ref_class {
          if method_instance.class_member.name == "println" {
            println(frame, &method_ref.member_ref.descriptor);
            return;
          }
          panic!("java.lang.NullPointerException");
        }

        let ref_class_instance;
        if let Slot::RefObject(Some(object)) = ref_class {
          ref_class_instance = object.borrow().class.clone();
        } else {
          panic!("ref class error!");
        }

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

        method_invoke::invoke_method(frame, method_to_be_invoke);
      }
    }
  }
}

fn println(frame: &mut Frame, descriptor: &String) {
  match descriptor.as_str() {
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
    "(Ljava/lang/String;)V" => {
      let j_str = frame.operand_stack.pop_ref().expect("not a ref");
      let r_sting = rs_string(&j_str.borrow());
      println!("{}", r_sting);
    }
    _ => panic!("println: {}", descriptor),
  }
  frame.operand_stack.pop_ref();
}
