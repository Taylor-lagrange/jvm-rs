use super::class::*;
use super::refs::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub enum ConstantInfoRunTime<'a> {
    Class(ClassRef<'a>),
    Fieldref(FieldRef<'a>),
    Methodref(MethodRef<'a>),
    InterfaceMethodref(InterfaceMethodRef<'a>),
    String(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Nil, //for empty
}

pub struct ConstantPool<'a> {
    pub consts: Vec<ConstantInfoRunTime<'a>>,
    pub class: Weak<RefCell<Class<'a>>>,
}

impl<'a> ConstantPool<'a> {
    pub fn new(
        class: Weak<RefCell<Class<'a>>>,
        pool: &crate::classfile::constant_pool::ConstantPool,
    ) -> Rc<RefCell<ConstantPool<'a>>> {
        let cp = Rc::new(RefCell::new(ConstantPool {
            consts: Vec::new(),
            class: class,
        }));
        let mut v = Vec::with_capacity(pool.pool_count as usize);
        for i in 0..(pool.pool_count as usize) {
            use crate::classfile::constant_pool::*;
            match pool.pool[i] {
                ConstantInfo::Class { .. } => v.push(ConstantInfoRunTime::Class(ClassRef::new(
                    Rc::downgrade(&cp),
                    &pool,
                    pool.pool[i].clone(),
                ))),
                ConstantInfo::Fieldref { .. } => v.push(ConstantInfoRunTime::Fieldref(
                    FieldRef::new(Rc::downgrade(&cp), &pool, pool.pool[i].clone()),
                )),
                ConstantInfo::Methodref { .. } => v.push(ConstantInfoRunTime::Methodref(
                    MethodRef::new(Rc::downgrade(&cp), &pool, pool.pool[i].clone()),
                )),
                ConstantInfo::InterfaceMethodref { .. } => {
                    v.push(ConstantInfoRunTime::InterfaceMethodref(
                        InterfaceMethodRef::new(Rc::downgrade(&cp), &pool, pool.pool[i].clone()),
                    ))
                }
                ConstantInfo::String { string_index } => v.push(ConstantInfoRunTime::String(
                    pool.get_utf8(string_index).to_owned(),
                )),
                ConstantInfo::Integer { val } => v.push(ConstantInfoRunTime::Integer(val)),
                ConstantInfo::Float { val } => v.push(ConstantInfoRunTime::Float(val)),
                ConstantInfo::Long { val } => {
                    v.push(ConstantInfoRunTime::Long(val));
                }
                ConstantInfo::Double { val } => {
                    v.push(ConstantInfoRunTime::Double(val));
                }
                _ => v.push(ConstantInfoRunTime::Nil),
            };
        }
        cp.borrow_mut().consts = v;
        cp
    }
    pub fn get_constant_info(&mut self, index: usize) -> &mut ConstantInfoRunTime<'a> {
        let x = self.consts.get_mut(index);
        match x {
            Some(val) => val,
            None => panic!("No constants at index {}", index),
        }
    }
}
