use super::constant_pool::ConstantInfoRunTime;
use crate::classfile::attribute_info::ExceptionTableEntry;
use crate::runtime::heap::class::Class;
use crate::runtime::heap::constant_pool::ConstantPool;
use crate::runtime::heap::refs::ClassRef;
use std::alloc::handle_alloc_error;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct ExceptionHandler<'a> {
    start_pc: usize,
    end_pc: usize,
    pub(crate) handler_pc: usize,
    catch_type: Option<ClassRef<'a>>,
}

pub type ExceptionTable<'a> = Vec<ExceptionHandler<'a>>;

impl<'a> ExceptionHandler<'a> {
    pub fn new(
        exception_table: Vec<ExceptionTableEntry>,
        pool: &Rc<RefCell<ConstantPool<'a>>>,
    ) -> ExceptionTable<'a> {
        let mut table = Vec::with_capacity(exception_table.len());
        for i in 0..exception_table.len() {
            table.push(ExceptionHandler {
                start_pc: exception_table[i].start_pc as usize,
                end_pc: exception_table[i].end_pc as usize,
                handler_pc: exception_table[i].handler_pc as usize,
                catch_type: ExceptionHandler::find_catch_type(exception_table[i].catch_type, pool),
            })
        }
        table
    }
    // catch_type 为 0 表示 catch all，所以用了个 Option，如果其指向的内容为 None，直接表示 catch all
    fn find_catch_type(index: u16, pool: &Rc<RefCell<ConstantPool<'a>>>) -> Option<ClassRef<'a>> {
        if index != 0 {
            if let ConstantInfoRunTime::Class(r) =
                pool.borrow_mut().get_constant_info(index as usize)
            {
                return Some(r.clone());
            }
            panic!("invalid index!");
        }
        None
    }
    pub fn find_exception_handler(
        table: &ExceptionTable<'a>,
        class: Rc<RefCell<Class<'a>>>,
        pc: usize,
    ) -> Option<ExceptionHandler<'a>> {
        for handler in table.iter() {
            // jvm: The start_pc is inclusive and end_pc is exclusive
            if handler.start_pc <= pc && pc < handler.end_pc {
                if handler.catch_type.is_none() {
                    return Some(handler.clone());
                }
                let catch_class = handler
                    .catch_type
                    .clone()
                    .unwrap()
                    .sym_ref
                    .resolved_class()
                    .upgrade()
                    .unwrap();
                if Rc::ptr_eq(&class, &catch_class)
                    || class.borrow().is_sub_class_of(Rc::downgrade(&catch_class))
                {
                    return Some(handler.clone());
                }
            }
        }
        None
    }
}
