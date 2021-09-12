use super::access_flags::*;
use super::class_loader::*;
use super::class_name_helper::*;
use super::constant_pool::*;
use super::field::*;
use super::method::*;
use crate::classfile::class_file::*;
use crate::runtime::heap::object::Object;
use crate::runtime::local_vars::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default, Clone)]
pub struct Class<'a> {
    pub access_flags: u16,
    pub name: String,
    pub super_class_name: String,
    pub interface_names: Vec<String>,
    pub constant_pool: Option<Rc<RefCell<ConstantPool<'a>>>>,
    pub fields: Vec<Rc<RefCell<Field<'a>>>>,
    pub methods: Vec<Rc<RefCell<Method<'a>>>>,
    pub loader: Weak<RefCell<ClassLoader<'a>>>,
    pub super_class: Weak<RefCell<Class<'a>>>,
    pub interfaces: Vec<Weak<RefCell<Class<'a>>>>,
    pub instance_slot_count: u32,
    pub static_slot_count: u32,
    pub static_vars: StaticFinalVar<'a>,
    pub init_started: bool,
}

impl<'a> Class<'a> {
    pub fn new_class(cf: ClassFile) -> Rc<RefCell<Class<'a>>> {
        let class = Rc::new(RefCell::new(Class {
            access_flags: cf.access_flags,
            name: cf.class_name(),
            super_class_name: cf.super_class_name(),
            interface_names: cf.interface_names(),
            ..Default::default()
        }));
        {
            let mut class_instance = class.borrow_mut();
            class_instance.constant_pool =
                Some(ConstantPool::new(Rc::downgrade(&class), &cf.constant_pool));
            class_instance.fields =
                Field::new_fields(Rc::downgrade(&class), &cf.constant_pool, cf.fields);
            class_instance.methods =
                Method::new_methods(Rc::downgrade(&class), &cf.constant_pool, cf.methods);
        }
        class
    }
    pub fn init_started(class: &Weak<RefCell<Class>>) -> bool {
        let we = class.clone().upgrade();
        if we.is_none() {
            return false;
        }
        let rc = we.unwrap();
        let class_instance = rc.borrow();
        class_instance.init_started
    }
    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }
    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }
    pub fn is_super(&self) -> bool {
        self.access_flags & ACC_SUPER != 0
    }
    pub fn is_interface(&self) -> bool {
        self.access_flags & ACC_INTERFACE != 0
    }
    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }
    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }
    pub fn is_annotation(&self) -> bool {
        self.access_flags & ACC_ANNOTATION != 0
    }
    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }
    pub fn is_accessible_to(&self, c: &Class) -> bool {
        self.is_public() || self.get_package_name() == c.get_package_name()
    }
    pub fn get_package_name(&self) -> String {
        let i = self.name.rfind("/");
        match i {
            Some(index) => {
                let (s, _) = self.name.split_at(index);
                s.to_string()
            }
            None => "".to_string(),
        }
    }
    pub fn get_field(
        &self,
        name: &String,
        descriptor: &String,
        is_static: bool,
    ) -> Rc<RefCell<Field<'a>>> {
        for field in self.fields.iter() {
            if field.borrow().class_member.is_static() == is_static
                && field.borrow().class_member.name == *name
                && field.borrow().class_member.descriptor == *descriptor
            {
                return field.clone();
            }
        }
        //recursive find field in class, so it not violate the borrow-checker
        let super_class = self.super_class.clone().upgrade();
        if super_class.is_none() {
            panic!("no field match require in current class!");
        }
        let rc = super_class.unwrap();
        return rc.borrow().get_field(name, descriptor, is_static);
    }
    fn get_static_method(&self, name: &String, descriptor: &String) -> Weak<RefCell<Method<'a>>> {
        for info in self.methods.iter() {
            if info.borrow().class_member.is_static()
                && info.borrow().class_member.name == *name
                && info.borrow().class_member.descriptor == *descriptor
            {
                return Rc::downgrade(info);
            }
        }
        Weak::new()
    }
    pub fn get_main_method(&self) -> Weak<RefCell<Method<'a>>> {
        self.get_static_method(&"main".to_string(), &"([Ljava/lang/String;)V".to_string())
    }
    pub fn get_clinit_method(&self) -> Weak<RefCell<Method<'a>>> {
        self.get_static_method(&"<clinit>".to_string(), &"()V".to_string())
    }
    // 数组可以强制转换成 Object类型（因为数组的超类是 Object）
    // 数组可以强制转换成 Cloneable 和 Serializable 类型（因为数组实现了这两个接口）
    // 如果下⾯两个条件之⼀成⽴，类型为[]SC的数组可以强制转换成类型为[]TC的数组
    // 1. TC 和 SC 是同⼀个基本类型
    // 2. TC 和 SC 都是引用类型，且 SC 可以强制转换成 TC
    // TODO: 需要修改 is_assignable_from 的逻辑从而让 instanceof 和 checkcast 指令正常工作
    pub fn is_assignable_from(&self, iface: Weak<RefCell<Class<'a>>>) -> bool {
        let i = iface.upgrade().unwrap();
        let class = i.borrow();
        if self.name == class.name {
            return true;
        }
        let temp_self_class = Rc::new(RefCell::new(self.clone()));
        if !self.is_interface() {
            class.is_sub_class_of(Rc::downgrade(&temp_self_class))
        } else {
            class.is_implements(Rc::downgrade(&temp_self_class))
        }
    }
    pub fn is_array(&self) -> bool {
        self.name.starts_with('[')
    }
    pub fn is_sub_class_of(&self, class: Weak<RefCell<Class<'a>>>) -> bool {
        let mut super_class = self.super_class.clone();
        while let Option::Some(c) = super_class.upgrade() {
            if c.borrow().name == class.upgrade().unwrap().borrow().name {
                return true;
            }
            super_class = c.borrow().super_class.clone();
        }
        false
    }
    pub fn is_implements(&self, iface: Weak<RefCell<Class<'a>>>) -> bool {
        if self.is_sub_interface_of(iface.clone()) {
            return true;
        }
        let mut super_class = self.super_class.clone();
        while let Option::Some(c) = super_class.upgrade() {
            if self.is_sub_interface_of(iface.clone()) {
                return true;
            }
            super_class = c.borrow().super_class.clone();
        }
        false
    }
    fn is_sub_interface_of(&self, iface: Weak<RefCell<Class<'a>>>) -> bool {
        for interface in self.interfaces.iter() {
            let rc = interface.upgrade().unwrap();
            let class = rc.borrow();
            let i_rc = iface.clone().upgrade().unwrap();
            if class.name == i_rc.borrow().name || class.is_sub_interface_of(iface.clone()) {
                return true;
            }
        }
        false
    }
    pub fn get_array_class(class: Weak<RefCell<Class>>) -> Weak<RefCell<Class>> {
        let loader;
        let array_class_name;
        {
            let rc = class.upgrade().unwrap();
            loader = rc.borrow().loader.clone();
            array_class_name = get_array_class_name(rc.borrow().name.to_owned());
        }
        ClassLoader::load_class(loader, &array_class_name)
    }
    pub fn component_class(class: Weak<RefCell<Class>>) -> Weak<RefCell<Class>> {
        let loader;
        let array_class_name;
        {
            let rc = class.upgrade().unwrap();
            loader = rc.borrow().loader.clone();
            array_class_name = get_component_class_name(rc.borrow().name.to_owned());
        }
        ClassLoader::load_class(loader, &array_class_name)
    }
}
