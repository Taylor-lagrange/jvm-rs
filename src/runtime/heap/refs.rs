use super::class::*;
use super::class_loader::*;
use super::constant_pool::*;
use super::field::*;
use super::method::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub struct SymRef<'a> {
  pub constant_pool: Weak<RefCell<ConstantPool<'a>>>,
  pub class_name: String,
  pub class: Weak<RefCell<Class<'a>>>,
}

impl<'a> SymRef<'a> {
  pub fn resolved_class(&mut self) -> Weak<RefCell<Class<'a>>> {
    let class = self.class.upgrade();
    if let Option::None = class {
      let pool_rc = self.constant_pool.upgrade().unwrap();
      let class_rc = pool_rc.borrow_mut().class.clone();
      let class = class_rc.upgrade().unwrap();
      let cp_class = class.borrow_mut();
      self.class = ClassLoader::load_class(cp_class.loader.clone(), &self.class_name);
      if !self
        .class
        .upgrade()
        .unwrap()
        .borrow()
        .is_accessible_to(&cp_class)
      {
        panic!("java.lang.IllegalAccessError");
      }
    }
    self.class.clone()
  }
}

#[derive(Default)]
pub struct MemberRef<'a> {
  pub sym_ref: SymRef<'a>,
  pub name: String,
  pub descriptor: String,
}

impl<'a> MemberRef<'a> {
  fn copy_member_ref_info(
    &mut self,
    pool: &crate::classfile::constant_pool::ConstantPool, //use pool because ref_info only record the index of const
    ref_info: crate::classfile::constant_pool::ConstantInfo,
  ) {
    let mut assign = |class_index, name_and_type_index| {
      self.sym_ref.class_name = pool.get_class_name(class_index).to_owned();
      let (name, des) = pool.get_name_and_type(name_and_type_index);
      self.name = name.to_owned();
      self.descriptor = des.to_owned();
    };
    match ref_info {
      crate::classfile::constant_pool::ConstantInfo::Methodref {
        class_index,
        name_and_type_index,
      } => assign(class_index, name_and_type_index),
      crate::classfile::constant_pool::ConstantInfo::InterfaceMethodref {
        class_index,
        name_and_type_index,
      } => assign(class_index, name_and_type_index),
      crate::classfile::constant_pool::ConstantInfo::Fieldref {
        class_index,
        name_and_type_index,
      } => assign(class_index, name_and_type_index),
      _ => {}
    };
  }
}

#[derive(Default)]
pub struct MethodRef<'a> {
  pub member_ref: MemberRef<'a>,
  pub methods: Weak<RefCell<Method<'a>>>,
}

impl<'a> MethodRef<'a> {
  pub fn new(
    cp: Weak<RefCell<ConstantPool<'a>>>,
    pool: &crate::classfile::constant_pool::ConstantPool,
    ref_info: crate::classfile::constant_pool::ConstantInfo,
  ) -> MethodRef<'a> {
    let mut mr: MethodRef<'a> = Default::default();
    mr.member_ref.sym_ref.constant_pool = cp;
    mr.member_ref.copy_member_ref_info(pool, ref_info);
    mr
  }
}

#[derive(Default)]
pub struct InterfaceMethodRef<'a> {
  pub member_ref: MemberRef<'a>,
  pub methods: Weak<RefCell<Method<'a>>>,
}

impl<'a> InterfaceMethodRef<'a> {
  pub fn new(
    cp: Weak<RefCell<ConstantPool<'a>>>,
    pool: &crate::classfile::constant_pool::ConstantPool,
    ref_info: crate::classfile::constant_pool::ConstantInfo,
  ) -> InterfaceMethodRef<'a> {
    let mut mr: InterfaceMethodRef<'a> = Default::default();
    mr.member_ref.sym_ref.constant_pool = cp;
    mr.member_ref.copy_member_ref_info(pool, ref_info);
    mr
  }
}

#[derive(Default)]
pub struct FieldRef<'a> {
  pub member_ref: MemberRef<'a>,
  pub field: Weak<RefCell<Field<'a>>>,
}

impl<'a> FieldRef<'a> {
  pub fn new(
    cp: Weak<RefCell<ConstantPool<'a>>>,
    pool: &crate::classfile::constant_pool::ConstantPool,
    ref_info: crate::classfile::constant_pool::ConstantInfo,
  ) -> FieldRef<'a> {
    let mut mr: FieldRef<'a> = Default::default();
    mr.member_ref.sym_ref.constant_pool = cp;
    mr.member_ref.copy_member_ref_info(pool, ref_info);
    mr
  }
  pub fn resolve_field(&mut self) -> Weak<RefCell<Field<'a>>> {
    if let Option::None = self.field.upgrade() {
      let c = self.member_ref.sym_ref.resolved_class();
      let field = FieldRef::lookup_field(c, &self.member_ref.name, &self.member_ref.descriptor);
      if let Option::None = field.upgrade() {
        panic!("java.lang.NoSuchFieldError");
      }
      let rc = self
        .member_ref
        .sym_ref
        .constant_pool
        .clone()
        .upgrade()
        .unwrap();
      if !field
        .upgrade()
        .unwrap()
        .borrow()
        .class_member
        .is_accessible_to(rc.borrow().class.clone())
      {
        panic!("java.lang.IllegalAccessError");
      }
      self.field = field;
    }
    self.field.clone()
  }
  fn lookup_field(
    c: Weak<RefCell<Class<'a>>>,
    name: &String,
    descriptor: &String,
  ) -> Weak<RefCell<Field<'a>>> {
    let rc = c.upgrade().unwrap();
    let class = rc.borrow();
    for info in class.fields.iter() {
      if info.borrow().class_member.descriptor == *descriptor
        && info.borrow().class_member.name == *name
      {
        return Rc::downgrade(&info);
      }
    }
    for info in class.interfaces.iter() {
      return FieldRef::lookup_field(c, name, descriptor);
    }
    if let Some(super_class) = class.super_class.upgrade() {
      return FieldRef::lookup_field(Rc::downgrade(&super_class), name, descriptor);
    }
    Weak::new()
  }
}

#[derive(Default)]
pub struct ClassRef<'a> {
  pub sym_ref: SymRef<'a>,
}

impl<'a> ClassRef<'a> {
  pub fn new(
    cp: Weak<RefCell<ConstantPool<'a>>>,
    pool: &crate::classfile::constant_pool::ConstantPool,
    ref_info: crate::classfile::constant_pool::ConstantInfo,
  ) -> ClassRef<'a> {
    let mut cf: ClassRef<'a> = Default::default();
    cf.sym_ref.constant_pool = cp;
    if let crate::classfile::constant_pool::ConstantInfo::Class { name_index } = ref_info {
      cf.sym_ref.class_name = pool.get_utf8(name_index).to_owned();
    }
    cf
  }
}