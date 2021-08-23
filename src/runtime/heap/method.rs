use super::access_flags::*;
use super::class::*;
use super::class_member::*;
use crate::classfile::attribute_info::*;
use crate::classfile::constant_pool::*;
use crate::classfile::member_info::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default,Clone)]
pub struct Method<'a> {
  pub class_member: ClassMember<'a>,
  pub max_stack: u32,
  pub max_locals: u32,
  pub code: Vec<u8>,
}

impl<'a> Method<'a> {
  pub fn new_methods(
    class: Weak<RefCell<Class<'a>>>,
    pool: &ConstantPool,
    cf_methods: Vec<MemberInfo>,
  ) -> Vec<Rc<RefCell<Method<'a>>>> {
    let mut methods = Vec::with_capacity(cf_methods.len());
    for info in cf_methods.iter() {
      methods.push(Rc::new(RefCell::new(Method::new(
        class.clone(),
        pool,
        info,
      ))));
    }
    methods
  }
  pub fn new(
    class: Weak<RefCell<Class<'a>>>,
    pool: &ConstantPool,
    member_info: &MemberInfo,
  ) -> Method<'a> {
    let mut method = Method {
      class_member: ClassMember::new(pool, class, member_info),
      ..Default::default()
    };
    if let AttributeInfo::Code {
      max_stack,
      max_locals,
      code,
      exception_table,
      attributes,
    } = member_info.code_attribute()
    {
      method.code = code;
      method.max_locals = max_locals as u32;
      method.max_stack = max_stack as u32;
    }
    method
  }

  pub fn is_synchronized(&self) -> bool {
    self.class_member.access_flags & ACC_SYNCHRONIZED != 0
  }
  pub fn is_bridge(&self) -> bool {
    self.class_member.access_flags & ACC_BRIDGE != 0
  }
  pub fn is_varargs(&self) -> bool {
    self.class_member.access_flags & ACC_VARARGS != 0
  }
  pub fn is_native(&self) -> bool {
    self.class_member.access_flags & ACC_NATIVE != 0
  }
  pub fn is_abstract(&self) -> bool {
    self.class_member.access_flags & ACC_ABSTRACT != 0
  }
  pub fn is_strict(&self) -> bool {
    self.class_member.access_flags & ACC_STRICT != 0
  }
}
