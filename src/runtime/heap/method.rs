use super::access_flags::*;
use super::class::*;
use super::class_member::*;
use super::method_descriptor_parser::*;
use crate::classfile::attribute_info::*;
use crate::classfile::constant_pool::*;
use crate::classfile::member_info::*;
use crate::runtime::heap::exception_table::{ExceptionHandler, ExceptionTable};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default, Clone)]
pub struct Method<'a> {
    pub class_member: ClassMember<'a>,
    pub max_stack: u32,
    pub max_locals: u32,
    pub code: Rc<Vec<u8>>,
    pub arg_slot_count: u32,
    pub exception_table: ExceptionTable<'a>,
    pub line_number_table: Vec<LineNumberTableEntry>,
    pub parsed_descriptor: MethodDescriptor,
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
        let pool_run_time;
        {
            let rc = class.clone().upgrade().unwrap();
            pool_run_time = rc.borrow().constant_pool.clone().unwrap();
        }
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
            method.code = Rc::new(code);
            method.max_locals = max_locals as u32;
            method.max_stack = max_stack as u32;
            method.exception_table = ExceptionHandler::new(exception_table, &pool_run_time);
            for attr in attributes {
                if let AttributeInfo::LineNumberTable { line_number_table } = attr {
                    method.line_number_table = line_number_table;
                }
            }
        }
        method.parsed_descriptor =
            MethodDescriptorParser::parse(method.class_member.descriptor.clone());
        method.calc_arg_slot_count();
        if method.is_native() {
            method.inject_code_attribute()
        }
        method
    }
    pub fn find_exception_handler(&self, class: Rc<RefCell<Class<'a>>>, pc: usize) -> i32 {
        let handler = ExceptionHandler::find_exception_handler(&self.exception_table, class, pc);
        if handler.is_some() {
            return handler.unwrap().handler_pc as i32;
        }
        -1
    }
    // ??????????????? class ??????????????? Code ????????????????????????
    // maxStack ??? maxLocals ???????????????????????????????????????????????????
    // ?????????????????????????????????????????????????????? maxStack ???????????????
    // 4??????????????????????????????????????????????????????????????????????????????
    // argSlotCount??????maxLocals?????????????????????code???????????????
    // ???????????????????????????????????????????????????0xFE???????????????????????????
    // ????????????????????????????????????????????????

    // ??????????????? 254???0xfe?????? 255???0xff???????????????????????? impdep1 ??? impdep2 ???????????????????????? ???????????? ??? ???????????????
    // Although these opcodes have been reserved, they may be used only inside a Java Virtual Machine implementation. They cannot appear in valid class files.
    // ????????????????????????JVM?????????????????????????????????????????? class ????????????????????????????????????????????????
    // ?????????????????? return ???????????????????????? native???????????????????????????????????????????????????????????????
    fn inject_code_attribute(&mut self) {
        self.max_stack = 4;
        self.max_locals = self.arg_slot_count;
        match self.parsed_descriptor.return_type.chars().nth(0).unwrap() {
            'V' => self.code = Rc::new(vec![0xfe, 0xb1]), // return
            'L' => self.code = Rc::new(vec![0xfe, 0xb0]), // areturn
            'D' => self.code = Rc::new(vec![0xfe, 0xaf]), // dreturn
            'F' => self.code = Rc::new(vec![0xfe, 0xae]), // freturn
            'J' => self.code = Rc::new(vec![0xfe, 0xad]), // lreturn
            _ => self.code = Rc::new(vec![0xfe, 0xac]),   // ireturn
        }
    }

    fn calc_arg_slot_count(&mut self) {
        for param in self.parsed_descriptor.parameter_type.iter() {
            self.arg_slot_count += 1;
            if param == "J" || param == "D" {
                self.arg_slot_count += 1;
            }
        }
        if !self.class_member.is_static() {
            self.arg_slot_count += 1; // `this` reference
        }
    }
    pub fn get_line_number(&self, pc: usize) -> i32 {
        if self.is_native() {
            return -2;
        }
        if self.line_number_table.len() == 0 {
            return -1;
        }
        for i in (0..self.line_number_table.len()).rev() {
            let entry = &self.line_number_table[i];
            if pc >= (*entry).start_pc as usize {
                return (*entry).line_number as i32;
            }
        }
        -1
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
