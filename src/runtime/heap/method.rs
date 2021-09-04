use super::access_flags::*;
use super::class::*;
use super::class_member::*;
use super::method_descriptor_parser::*;
use crate::classfile::attribute_info::*;
use crate::classfile::constant_pool::*;
use crate::classfile::member_info::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default, Clone)]
pub struct Method<'a> {
  pub class_member: ClassMember<'a>,
  pub max_stack: u32,
  pub max_locals: u32,
  pub code: Rc<Vec<u8>>,
  pub arg_slot_count: u32,
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
      ..
    } = member_info.code_attribute()
    {
      method.code = Rc::new(code);
      method.max_locals = max_locals as u32;
      method.max_stack = max_stack as u32;
    }
    method.calc_arg_slot_count();
    method
  }

  fn calc_arg_slot_count(&mut self) {
    let parsed_descriptor = MethodDescriptorParser::parse(self.class_member.descriptor.clone());
    for param in parsed_descriptor.parameter_type {
      self.arg_slot_count += 1;
      if param == "J" || param == "D" {
        self.arg_slot_count += 1;
      }
    }
    if !self.class_member.is_static() {
      self.arg_slot_count += 1; // `this` reference
    }
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

pub fn lookup_method_in_class<'a>(
  c: Weak<RefCell<Class<'a>>>,
  name: &String,
  descriptor: &String,
) -> Weak<RefCell<Method<'a>>> {
  let mut iter_class = c;
  loop {
    let we = iter_class.upgrade();
    if we.is_none() {
      break;
    }
    let rc = we.unwrap();
    let class = rc.borrow();
    for info in class.methods.iter() {
      if info.borrow().class_member.descriptor == *descriptor
        && info.borrow().class_member.name == *name
      {
        return Rc::downgrade(&info);
      }
    }
    iter_class = class.super_class.clone();
  }
  Weak::new()
}

pub fn lookup_method_in_interfaces<'a>(
  c: Weak<RefCell<Class<'a>>>,
  name: &String,
  descriptor: &String,
) -> Weak<RefCell<Method<'a>>> {
  let rc = c.upgrade().unwrap();
  let class = rc.borrow();
  for iface in class.interfaces.iter() {
    let rc = iface.clone().upgrade().unwrap();
    let iface_class = rc.borrow();
    for info in iface_class.methods.iter() {
      if info.borrow().class_member.descriptor == *descriptor
        && info.borrow().class_member.name == *name
      {
        return Rc::downgrade(&info);
      }
    }
    let method = lookup_method_in_interfaces(iface.clone(), name, descriptor);
    if method.upgrade().is_some() {
      return method;
    }
  }
  Weak::new()
}
