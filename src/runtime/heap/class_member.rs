use super::access_flags::*;
use super::class::*;
use crate::classfile::constant_pool::*;
use crate::classfile::member_info::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default, Clone)]
pub struct ClassMember<'a> {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub class: Weak<RefCell<Class<'a>>>,
}

impl<'a> ClassMember<'a> {
    pub fn new(
        pool: &ConstantPool,
        class: Weak<RefCell<Class<'a>>>,
        member_info: &MemberInfo,
    ) -> ClassMember<'a> {
        ClassMember {
            access_flags: member_info.access_flags,
            name: pool.get_utf8(member_info.name_index as usize).to_owned(),
            descriptor: pool
                .get_utf8(member_info.descriptor_index as usize)
                .to_owned(),
            class: class,
        }
    }
    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }
    pub fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }
    pub fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTECTED != 0
    }
    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }
    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }
    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }
    pub fn is_accessible_to(&self, class: Weak<RefCell<Class<'a>>>) -> bool {
        if self.is_public() {
            return true;
        }
        let access_class = class.upgrade().unwrap();
        let self_class = self.class.upgrade().unwrap();
        if self.is_protected() {
            return Rc::ptr_eq(&access_class, &self_class)
                || access_class.borrow().is_sub_class_of(self.class.clone())
                || self_class.borrow().get_package_name()
                    == access_class.borrow().get_package_name();
        }
        if !self.is_private() {
            return self_class.borrow().get_package_name()
                == access_class.borrow().get_package_name();
        }
        Rc::ptr_eq(&access_class, &self_class)
    }
}
