use super::access_flags::*;
use super::class::*;
use super::constant_pool::*;
use crate::classfile::class_file::*;
use crate::classpath::classpath::*;
use crate::classpath::entry::Entry;
use crate::runtime::local_vars::*;
use crate::runtime::heap::string_pool::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct ClassLoader<'a> {
  class_path: Classpath,
  class_map: HashMap<String, Rc<RefCell<Class<'a>>>>,
}

impl<'a> ClassLoader<'a> {
  pub fn new(cp: Classpath) -> Rc<RefCell<ClassLoader<'a>>> {
    Rc::new(RefCell::new(ClassLoader {
      class_path: cp,
      class_map: HashMap::new(),
    }))
  }
  pub fn load_class(
    loader: Weak<RefCell<ClassLoader<'a>>>,
    name: &String,
  ) -> Weak<RefCell<Class<'a>>> {
    {
      let mut class = Weak::new();
      {
        let rc = loader.clone().upgrade().unwrap();
        let loader_instance = rc.borrow();
        let value = loader_instance.class_map.get(name);
        if value.is_some() {
          class = Rc::downgrade(value.unwrap());
        }
      }
      if let Some(..) = class.upgrade() {
        return class;
      }
    }
    if name.starts_with('[') {
      return ClassLoader::load_array_class(loader, name);
    }
    // use {} to mark the borrow range of loader_instance,
    // because loader will be borrow once again in load_non_array_class
    // so before excute load_non_array_class, the borrow of loader must finish
    ClassLoader::load_non_array_class(loader, name)
  }
  fn load_array_class(
    loader: Weak<RefCell<ClassLoader<'a>>>,
    name: &String,
  ) -> Weak<RefCell<Class<'a>>> {
    let class = Rc::new(RefCell::new(Class {
      access_flags: ACC_PUBLIC,
      name: name.to_owned(),
      loader: loader.clone(),
      init_started: true,
      super_class: ClassLoader::load_class(loader.clone(), &"java/lang/Object".to_string()),
      interfaces: vec![
        ClassLoader::load_class(loader.clone(), &"java/lang/Cloneable".to_string()),
        ClassLoader::load_class(loader.clone(), &"java/io/Serializable".to_string()),
      ],
      ..Default::default()
    }));
    {
      let rc = loader.upgrade().unwrap();
      let mut class_loader = rc.borrow_mut();
      class_loader
        .class_map
        .insert(class.borrow().name.clone(), class.clone());
    }
    Rc::downgrade(&class)
  }
  fn load_non_array_class(
    loader: Weak<RefCell<ClassLoader<'a>>>,
    name: &String,
  ) -> Weak<RefCell<Class<'a>>> {
    let classbyte;
    let class;
    {
      let rc = loader.upgrade().unwrap();
      let mut class_loader = rc.borrow_mut();
      classbyte = class_loader.read_class(name);
      class = class_loader.parse_class(classbyte);
      class.borrow_mut().loader = loader.clone();
    }
    ClassLoader::resolve_super_class(Rc::downgrade(&class));
    ClassLoader::resolve_interfaces(Rc::downgrade(&class));
    {
      let rc = loader.upgrade().unwrap();
      let mut class_loader = rc.borrow_mut();
      class_loader
        .class_map
        .insert(class.borrow().name.clone(), class.clone());
    }
    ClassLoader::link(class.clone());
    Rc::downgrade(&class)
  }
  fn resolve_super_class(class: Weak<RefCell<Class<'a>>>) {
    let mut have_super = false;
    let mut loader_ref = Weak::new();
    let mut class_name = String::new();
    {
      let rc = class.upgrade().unwrap();
      let class_instance = rc.borrow();
      if class_instance.name != "java/lang/Object" {
        have_super = true;
        loader_ref = class_instance.loader.clone();
        class_name = class_instance.super_class_name.clone()
      }
    }
    if have_super {
      let super_class = ClassLoader::load_class(loader_ref, &class_name);
      let rc = class.upgrade().unwrap();
      let mut class_instance = rc.borrow_mut();
      class_instance.super_class = super_class;
    }
  }
  fn resolve_interfaces(class: Weak<RefCell<Class<'a>>>) {
    let rc = class.upgrade().unwrap();
    let mut class_instance = rc.borrow_mut();
    let mut v = Vec::with_capacity(class_instance.interfaces.len() as usize);
    if class_instance.interfaces.len() != 0 {
      for name in class_instance.interface_names.iter() {
        v.push(ClassLoader::load_class(class_instance.loader.clone(), name));
      }
    }
    class_instance.interfaces = v
  }
  fn link(class: Rc<RefCell<Class<'a>>>) {
    ClassLoader::calc_instance_field_slot_ids(class.clone());
    ClassLoader::calc_static_field_slot_ids(class.clone());
    ClassLoader::alloc_and_init_static_vars(class.clone());
  }
  fn calc_instance_field_slot_ids(class: Rc<RefCell<Class<'a>>>) {
    let mut slot_id = 0;
    if let Option::Some(c) = &class.borrow().super_class.upgrade() {
      slot_id = c.borrow().instance_slot_count;
    }
    for fd in class.borrow_mut().fields.iter_mut() {
      if !fd.borrow().class_member.is_static() {
        fd.borrow_mut().slot_id = slot_id;
        slot_id += 1;
        if fd.borrow().is_long_or_double() {
          slot_id += 1;
        }
      }
    }
    class.borrow_mut().instance_slot_count = slot_id;
  }
  fn calc_static_field_slot_ids(class: Rc<RefCell<Class<'a>>>) {
    let mut slot_id = 0;
    for fd in class.borrow_mut().fields.iter_mut() {
      if fd.borrow().class_member.is_static() {
        fd.borrow_mut().slot_id = slot_id;
        slot_id += 1;
        if fd.borrow().is_long_or_double() {
          slot_id += 1;
        }
      }
    }
    class.borrow_mut().static_slot_count = slot_id;
  }
  fn alloc_and_init_static_vars(class: Rc<RefCell<Class<'a>>>) {
    let mut class_instance = class.borrow_mut();
    let loader = class_instance.loader.clone();
    let mut v = StaticFinalVar::new(class_instance.static_slot_count as usize);
    let pool_clone = class_instance.constant_pool.clone().unwrap();
    let mut pool = pool_clone.borrow_mut();
    for fd in class_instance.fields.iter_mut() {
      let field = fd.borrow();
      if field.class_member.is_static() || field.class_member.is_final() {
        if field.const_value_index > 0 {
          match field.class_member.descriptor.as_str() {
            "Z" | "B" | "C" | "S" | "I" => {
              if let ConstantInfoRunTime::Integer(val) =
                pool.get_constant_info(field.const_value_index as usize)
              {
                v.set_int(field.slot_id as usize, *val);
              }
            }
            "J" => {
              if let ConstantInfoRunTime::Long(val) =
                pool.get_constant_info(field.const_value_index as usize)
              {
                v.set_long(field.slot_id as usize, *val);
              }
            }
            "F" => {
              if let ConstantInfoRunTime::Float(val) =
                pool.get_constant_info(field.const_value_index as usize)
              {
                v.set_float(field.slot_id as usize, *val);
              }
            }
            "D" => {
              if let ConstantInfoRunTime::Double(val) =
                pool.get_constant_info(field.const_value_index as usize)
              {
                v.set_double(field.slot_id as usize, *val);
              }
            }
            "Ljava/lang/String;" => {
              if let ConstantInfoRunTime::String(val) =
                pool.get_constant_info(field.const_value_index as usize)
              {
                let s_obj = j_string(loader.clone(), &val);
                v.set_ref(field.slot_id as usize, Some(s_obj));
              }
            },
            _ => panic!("unkown descriptor"),
          }
        }
      }
    }
    class_instance.static_vars = v;
  }
  fn read_class(&mut self, name: &String) -> Vec<u8> {
    let class = self.class_path.read_class(name.to_owned());
    match class {
      Ok(bytes) => bytes,
      Err(err) => panic!("{}", err),
    }
  }
  fn parse_class(&self, data: Vec<u8>) -> Rc<RefCell<Class<'a>>> {
    let classfile = ClassFile::parse(data);
    if let Result::Err(err) = classfile {
      panic!("{}", err);
    }
    Class::new_class(classfile.unwrap())
  }
}
