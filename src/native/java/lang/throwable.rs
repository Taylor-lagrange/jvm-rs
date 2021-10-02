use crate::runtime::heap::class::Class;
use crate::runtime::heap::class_loader::ClassLoader;
use crate::runtime::heap::object::{ObjectExtra, StackTraceElement};
use crate::runtime::thread::{Frame, Thread};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// private native Throwable fillInStackTrace(int dummy);
// (I)Ljava/lang/Throwable;
pub fn fill_in_stack_trace_impl(frame: &mut Frame) {
    let this = frame.local_vars.get_ref(0).unwrap();
    frame.operand_stack.push_ref(Some(this.clone()));
    // 由于栈顶两帧正在执⾏
    // fillInStackTrace（int）和fillInStackTrace（）⽅法，
    // 所以需要跳过这两帧。这两帧下⾯的⼏帧正在执⾏异常类
    // 的构造函数，所以也要跳过，具体要跳过多少帧数则要看
    // 异常类的继承层次。distanceToObject（）函数计算所
    // 需跳过的帧数
    let skip = distance_to_object(this.borrow().class.clone()) + 2;
    let thread = frame.thread.clone().upgrade().unwrap();
    let stes = create_stack_trace_elements(skip, thread);
    this.borrow_mut().extra = ObjectExtra::StackTrace(stes);
}

fn create_stack_trace_elements(skip: i32, thread: Rc<RefCell<Thread>>) -> Vec<StackTraceElement> {
    let frames = thread.borrow().get_frames();
    let mut stes = Vec::with_capacity(frames.len() - skip as usize);
    for i in skip as usize..frames.len() {
        let frame = frames[i].borrow();
        stes.push(StackTraceElement::new(&frame));
    }
    stes
}

fn distance_to_object(mut class: Weak<RefCell<Class>>) -> i32 {
    let mut distance = 0;
    loop {
        let rc = class.upgrade().unwrap();
        let c = rc.borrow().super_class.clone();
        if c.upgrade().is_none() {
            break;
        }
        distance += 1;
        class = c;
    }
    distance
}
