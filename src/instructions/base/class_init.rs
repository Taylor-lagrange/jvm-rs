use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn init_class<'a>(thread: Weak<RefCell<Thread<'a>>>, class: Weak<RefCell<Class<'a>>>) {
    {
        let rc = class.clone().upgrade().unwrap();
        rc.borrow_mut().init_started = true;
    }
    schedule_clinit(thread.clone(), class.clone());
    init_super_class(thread, class);
}

pub fn schedule_clinit<'a>(thread: Weak<RefCell<Thread<'a>>>, class: Weak<RefCell<Class<'a>>>) {
    let clinit;
    {
        let rc = class.upgrade().unwrap();
        clinit = rc.borrow().get_clinit_method();
    }
    if clinit.upgrade().is_some() {
        let frame = Thread::new_frame(thread.clone(), clinit.upgrade().unwrap());

        let rc = thread.upgrade().unwrap();
        let mut thread_instance = rc.borrow_mut();

        thread_instance.stack.push(frame);
    }
}

pub fn init_super_class<'a>(thread: Weak<RefCell<Thread<'a>>>, class: Weak<RefCell<Class<'a>>>) {
    let is_interface;
    let super_class;
    let init_started;
    {
        let rc = class.upgrade().unwrap();
        let class_instance = rc.borrow();
        is_interface = class_instance.is_interface();
        super_class = class_instance.super_class.clone();
        init_started = Class::init_started(&super_class);
    }

    if !is_interface && super_class.upgrade().is_some() && !init_started {
        init_class(thread, super_class);
    }
}
