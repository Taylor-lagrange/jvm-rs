use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::instructions::base::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::method::*;
use crate::runtime::operand_stack::*;
use crate::runtime::thread::*;
use std::rc::Rc;

pub struct INVOKE_SPECIAL {}

impl Index16Instruction for INVOKE_SPECIAL {}

impl Instruction for INVOKE_SPECIAL {
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
        if method_instance.class_member.name == "<init>"
          && !Rc::ptr_eq(
            &method_instance
              .class_member
              .class
              .clone()
              .upgrade()
              .unwrap(),
            &class_rc,
          )
        {
          panic!("java.lang.NoSuchMethodError");
        }
        if method_instance.class_member.is_static() {
          panic!("java.lang.IncompatibleClassChangeError");
        }
        let ref_class = frame
          .operand_stack
          .get_ref_from_top(method_instance.arg_slot_count as usize - 1);
        // make sure the class pointer `this` is not null
        if let Slot::RefObject(None) = ref_class {
          panic!("java.lang.NullPointerException");
        }

        // if resolvedMethod.IsProtected() &&
        // resolvedMethod.Class().IsSuperClassOf(currentClass) &&
        // resolvedMethod.Class().GetPackageName() != currentClass.GetPackageName() &&
        // ref.Class() != currentClass &&
        // !ref.Class().IsSubClassOf(currentClass) {
        //   panic("java.lang.IllegalAccessError")
        // }

        let mut method_to_be_invoke = method_rc.clone();
        {
          let class_instance_is_sub_class_of_current_class =
            class_instance.is_sub_class_of(frame.method.borrow_mut().class_member.class.clone());
          let rc = frame
            .method
            .borrow_mut()
            .class_member
            .class
            .clone()
            .upgrade()
            .unwrap();
          let current_class = rc.borrow();
          if current_class.is_super()
            && class_instance_is_sub_class_of_current_class
            && method_instance.class_member.name != "<init>"
          {
            let super_method = lookup_method_in_class(
              current_class.super_class.clone(),
              &method_instance.class_member.name,
              &method_instance.class_member.descriptor,
            );
            if super_method.upgrade().is_none() {
              panic!("java.lang.AbstractMethodError");
            }
            method_to_be_invoke = super_method.upgrade().unwrap();
            if method_to_be_invoke.borrow().is_abstract() {
              panic!("java.lang.AbstractMethodError");
            }
          }
        }
        method_invoke::invoke_method(frame, method_to_be_invoke);
      }
    }
  }
}
