use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::object::{Object, ObjectExtra};
use crate::runtime::heap::string_pool::rs_string;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ATHROW {}

impl NoOperandsInstruction for ATHROW {}

impl Instruction for ATHROW {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let ex = frame.operand_stack.pop_ref();
        if ex.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let thread = frame.thread.clone().upgrade().unwrap();
        let ex_obj = ex.unwrap();
        // find_and_goto_exception_handler 这个函数会顺着 thread 的各个 frame 找异常
        // 但当前栈帧的 frame 已经被借用，所以需要特判
        let mut current_find = false;
        let pc = frame.next_pc - 1;
        let handler_pc;
        {
            let class = ex_obj.borrow().class.clone().upgrade().unwrap();
            let rc = frame.method.clone();
            handler_pc = rc.borrow().find_exception_handler(class, pc);
        }
        if handler_pc > 0 {
            frame.operand_stack.clear();
            frame.operand_stack.push_ref(Some(ex_obj.clone()));
            frame.next_pc = handler_pc as usize;
            current_find = true;
        }
        if current_find {
            return;
        }
        // 特判完成
        if !find_and_goto_exception_handler(&thread, &ex_obj) {
            handle_uncaught_exception(thread, ex_obj);
        }
    }
}

fn find_and_goto_exception_handler<'a>(
    thread: &Rc<RefCell<Thread<'a>>>,
    ex: &Rc<RefCell<Object<'a>>>,
) -> bool {
    // 当前栈帧已经查过了，直接弹出
    thread.borrow_mut().stack.pop();
    loop {
        let frame = thread.borrow_mut().stack.top();
        let pc = frame.borrow().next_pc - 1;
        let handler_pc;
        {
            let class = ex.borrow().class.clone().upgrade().unwrap();
            let rc = frame.borrow().method.clone();
            handler_pc = rc.borrow().find_exception_handler(class, pc);
        }
        if handler_pc > 0 {
            frame.borrow_mut().operand_stack.clear();
            frame.borrow_mut().operand_stack.push_ref(Some(ex.clone()));
            frame.borrow_mut().next_pc = handler_pc as usize;
            return true;
        }
        thread.borrow_mut().stack.pop();
        if thread.borrow().stack.is_empty() {
            break;
        }
    }
    false
}

fn handle_uncaught_exception(thread: Rc<RefCell<Thread>>, ex: Rc<RefCell<Object>>) {
    thread.borrow_mut().clear_stack();
    let j_msg = ex.borrow().get_ref_var(
        &"detailMessage".to_string(),
        &"Ljava/lang/String;".to_string(),
    );
    let rs_msg = rs_string(&j_msg.unwrap());
    let java_name;
    {
        let rc = ex.borrow().class.clone().upgrade().unwrap();
        java_name = rc.borrow().name.clone();
    }
    println!("{}: {}", java_name, rs_msg);
    let ex_instance = ex.borrow();
    if let ObjectExtra::StackTrace(table) = &ex_instance.extra {
        for i in 0..table.len() {
            println!("\tat {}", table[i].to_string());
        }
    }
}
