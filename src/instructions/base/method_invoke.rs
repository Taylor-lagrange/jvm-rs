use crate::runtime::heap::method::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invoke_method<'a>(frame: &mut Frame<'a>, method: Rc<RefCell<Method<'a>>>) {
  let rc = frame.thread.clone().upgrade().unwrap();
  let mut thread = rc.borrow_mut();
  let new_frame = Thread::new_frame(frame.thread.clone(), method.clone());

  let method_rc = method.borrow();
  let arg_slot_count = method_rc.arg_slot_count;
  if arg_slot_count > 0 {
    for i in (0..arg_slot_count).rev() {
      let slot = frame.operand_stack.pop_slot();
      new_frame.borrow_mut().local_vars.set_slot(i as usize, slot);
    }
  }
  thread.stack.push(new_frame);

  // let method_instance = method.borrow();
  // if method_instance.is_native() {
  //   if method_instance.class_member.name == "registerNatives" {
  //     thread.stack.pop();
  //   } else {
  //     panic!("native method: {}\n", method_instance.class_member.name);
  //   }
  // }
}
