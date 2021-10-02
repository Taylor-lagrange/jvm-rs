use super::access_flags::*;
use super::class::*;
use super::class_name_helper::PRIMITIVE_TYPES;
use super::constant_pool::*;
use crate::classfile::class_file::*;
use crate::classpath::classpath::*;
use crate::classpath::entry::Entry;
use crate::runtime::heap::object::{Object, ObjectExtra};
use crate::runtime::heap::string_pool::*;
use crate::runtime::local_vars::*;
use log::debug;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct ClassLoader<'a> {
    class_path: Classpath,
    class_map: HashMap<String, Rc<RefCell<Class<'a>>>>,
}

impl<'a> ClassLoader<'a> {
    pub fn new(cp: Classpath) -> Rc<RefCell<ClassLoader<'a>>> {
        let loader = Rc::new(RefCell::new(ClassLoader {
            class_path: cp,
            class_map: HashMap::new(),
        }));
        ClassLoader::load_basic_classes(loader.clone());
        ClassLoader::load_primitive_classes(loader.clone());
        loader
    }
    // void.class int.class 这些类 jvm 运行时生成
    // 每个基本类型都有一个包装类，包装类里面有个静态常量叫TYPE, 存放 T.class
    // System.out.println(int.class) 最后类的访问会被修改为 getstatic 指令访问
    // 其他 xxxx.class 会被 instruction/constant/ldc 指令当作常量加载
    fn load_primitive_classes(loader: Rc<RefCell<ClassLoader<'a>>>) {
        for key in PRIMITIVE_TYPES.keys() {
            let class = Rc::new(RefCell::new(Class {
                access_flags: ACC_PUBLIC,
                name: key.to_string(),
                loader: Rc::downgrade(&loader),
                init_started: true,
                ..Default::default()
            }));
            let j_class = Object::new_object(&loader.borrow().class_map["java/lang/Class"]);
            j_class.borrow_mut().extra = ObjectExtra::Class(Rc::downgrade(&class));
            //之前忘记赋值导致调用get_primitive_class找出来的是空引用，最后导致空指针异常。
            class.borrow_mut().j_class = Some(j_class);
            loader.borrow_mut().class_map.insert(key.to_string(), class);
            debug!("primitive classes {} be loaded !", key);
        }
    }
    // loader 里面的 map 存着一堆 class 结构体（可以认为 loader 拥有所有 class 的所有权，其他结构体里存的 class 都是弱引用（防止循环依赖））
    // 每个 class 结构体都有一个 java/lang/Class 的 object 用于在反射的时候映射到一个 object 上
    // 但是这个 object 的类型是 java/lang/Class ，没有足够的信息去反射到一个具体的类型，所以在 object 里增加了一个 extra 字段
    // 存一个 class 的弱引用，指向这个 java/lang/Class 类型的 object 需要表达的类（也就是 loader 对应的 class）。
    fn load_basic_classes(loader: Rc<RefCell<ClassLoader<'a>>>) {
        let j_class =
            ClassLoader::load_class(Rc::downgrade(&loader), &"java/lang/Class".to_string())
                .upgrade()
                .unwrap();
        let mut instance_loader = loader.borrow_mut();
        for (_, class) in instance_loader.class_map.iter_mut() {
            let is_j_class;
            {
                is_j_class = class.borrow().j_class.is_none();
            }
            if is_j_class {
                let j_class_object = Object::new_object(&j_class);
                class.borrow_mut().j_class = Some(j_class_object.clone());
                j_class_object.borrow_mut().extra = ObjectExtra::Class(Rc::downgrade(class));
            }
        }
    }
    pub fn load_class(
        loader: Weak<RefCell<ClassLoader<'a>>>,
        name: &String,
    ) -> Weak<RefCell<Class<'a>>> {
        // use {} to mark the borrow range of loader_instance,
        // because loader will be borrow once again in load_non_array_class
        // so before execute load_non_array_class, the borrow of loader must finish
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
                // let instance = class.clone().upgrade().unwrap();
                // let j_class = instance.borrow().j_class.clone().unwrap();
                // let c = j_class.borrow_mut();
                // if let ObjectExtra::Nil=c.extra{
                //     debug!("what fuck");
                // }
                return class;
            }
        }
        debug!("load classes {} ...", name);
        let class = if name.starts_with('[') {
            ClassLoader::load_array_class(loader.clone(), name)
        } else {
            ClassLoader::load_non_array_class(loader.clone(), name)
        };
        class
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
        // assign class object (java/lang/Class) to the class, enable reflect to this class
        {
            let loader_instance = loader.upgrade().unwrap();
            ClassLoader::load_j_class(&loader_instance, &class);
            loader_instance
                .borrow_mut()
                .class_map
                .insert(class.borrow().name.clone(), class.clone());
        }
        Rc::downgrade(&class)
    }
    // 把 class 的 j_class 字段填好
    pub fn load_j_class(loader: &Rc<RefCell<ClassLoader<'a>>>, class: &Rc<RefCell<Class<'a>>>) {
        let load_instance = loader.borrow();
        let j_class = load_instance.class_map.get("java/lang/Class");
        if j_class.is_some() {
            let j_object = Object::new_object(j_class.unwrap());
            j_object.borrow_mut().extra = ObjectExtra::Class(Rc::downgrade(class));
            class.borrow_mut().j_class = Some(j_object);
        }
    }
    fn load_non_array_class(
        loader: Weak<RefCell<ClassLoader<'a>>>,
        name: &String,
    ) -> Weak<RefCell<Class<'a>>> {
        let class_byte;
        let class;
        {
            let rc = loader.upgrade().unwrap();
            let mut class_loader = rc.borrow_mut();
            class_byte = class_loader.read_class(name);
            class = class_loader.parse_class(class_byte);
            class.borrow_mut().loader = loader.clone();
        }
        ClassLoader::resolve_super_class(Rc::downgrade(&class));
        ClassLoader::resolve_interfaces(Rc::downgrade(&class));
        // assign class object (java/lang/Class) to the class, enable reflect to this class
        {
            let loader_instance = loader.upgrade().unwrap();
            ClassLoader::load_j_class(&loader_instance, &class);
            loader_instance
                .borrow_mut()
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
        if class_instance.interface_names.len() != 0 {
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
        debug!(
            "alloc and init static vars for class {}",
            class.borrow().name
        );
        let mut class_instance = class.borrow_mut();
        let loader = class_instance.loader.clone();
        let mut v = StaticFinalVar::new(class_instance.static_slot_count as usize);
        let pool_clone = class_instance.constant_pool.clone().unwrap();
        let mut pool = pool_clone.borrow_mut();
        for fd in class_instance.fields.iter_mut() {
            let field = fd.borrow();
            if field.class_member.is_static() || field.class_member.is_final() {
                if field.const_value_index > 0 {
                    // for basic static variable (like int double ...),the value can be assign at beginning
                    // but for static class reference, the value will be assign at clinit
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
                        }
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
