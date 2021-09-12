use super::class::*;
use crate::runtime::local_vars::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Byte = i8;
pub type Short = i16;
pub type Int = i32;
pub type Long = i64;
// 许多年前 Unicode 的提出者天真地以为 16 位定长的字符可以容纳地球上所有仍具活力的文字，Java 设计者也深以为然。
// 参考 Unicode 设计，Java 设计者认为完全可以设计一个双字节数据类型来表达所有 Unicode 字符
// 于是便有了今天的原始数据类型 char。但后来发现 65,536 个字符根本不足以表达所有文字，
// Java 5.0 版本既要支持 Unicode 4.0 同时要保证向后兼容性，不得不开始使用 UTF-16 作为内部编码方式，

// 但注意，在class⽂件中，字符串是以 MUTF8 格式保存的
pub type Char = u16;
pub type Float = f32;
pub type Double = f64;
pub type Ref<'a> = Rc<RefCell<Object<'a>>>;

// pub trait Array {
//   type Item;
//   fn len(&self) -> usize;
//   fn get(&self, index: usize) -> Option<Self::Item>;
//   fn set(&mut self, index: usize, item: Self::Item);
// }

// pub struct Bytes(Vec<Byte>);
// pub struct Shorts(Vec<Short>);
// pub struct Ints(Vec<Int>);
// pub struct Longs(Vec<Long>);
// pub struct Chars(Vec<Char>);
// pub struct Floats(Vec<Float>);
// pub struct Doubles(Vec<Double>);
// pub struct Refs<'a>(Vec<Ref<'a>>);

// impl Array for Bytes {
//   type Item = Byte;
//   fn len(&self) -> usize {
//     self.0.len()
//   }
//   fn get(&self, index: usize) -> Option<Self::Item> {
//     match self.0.get(index) {
//       Some(x) => Some(*x),
//       None => None,
//     }
//   }
//   fn set(&mut self, index: usize, item: Self::Item) {
//     self.0[index] = item
//   }
// }

// #[derive(Default)]
// pub struct ObjectData<'a> {
//   pub array_bytes: Vec<Byte>,
//   pub array_shorts: Vec<Short>,
//   pub array_ints: Vec<Int>,
//   pub array_longs: Vec<Long>,
//   pub array_chars: Vec<Char>,
//   pub array_floats: Vec<Float>,
//   pub array_doubles: Vec<Double>,
//   pub array_refs: Vec<Ref<'a>>,
//   pub field: FieldVar<'a>,
// }

// ObjectData {
//   field: FieldVar::new(class.upgrade().unwrap().borrow().instance_slot_count as usize),
//   ..Default::default()
// },

pub enum ObjectData<'a> {
    ArrayBytes(Vec<Byte>),
    ArrayShorts(Vec<Short>),
    ArrayInts(Vec<Int>),
    ArrayLongs(Vec<Long>),
    ArrayChars(Vec<Char>),
    ArrayFloats(Vec<Float>),
    ArrayDoubles(Vec<Double>),
    ArrayRefs(Vec<Ref<'a>>),
    Field(FieldVar<'a>),
    Nil,
}

pub struct Object<'a> {
    pub class: Weak<RefCell<Class<'a>>>,
    pub data: ObjectData<'a>,
}

impl<'a> Object<'a> {
    pub fn new(class: Weak<RefCell<Class<'a>>>) -> Object<'a> {
        Object {
            class: class.clone(),
            data: ObjectData::Field(FieldVar::new(
                class.upgrade().unwrap().borrow().instance_slot_count as usize,
            )),
        }
    }
    pub fn new_object(class: &Rc<RefCell<Class<'a>>>) -> Rc<RefCell<Object<'a>>> {
        Rc::new(RefCell::new(Object::new(Rc::downgrade(class))))
    }
    pub fn is_instance_of(&self, class: Weak<RefCell<Class<'a>>>) -> bool {
        class
            .upgrade()
            .unwrap()
            .borrow()
            .is_assignable_from(self.class.clone())
    }
    pub fn array_length(&self) -> usize {
        match &self.data {
            ObjectData::ArrayBytes(val) => val.len(),
            ObjectData::ArrayShorts(val) => val.len(),
            ObjectData::ArrayInts(val) => val.len(),
            ObjectData::ArrayLongs(val) => val.len(),
            ObjectData::ArrayChars(val) => val.len(),
            ObjectData::ArrayFloats(val) => val.len(),
            ObjectData::ArrayDoubles(val) => val.len(),
            ObjectData::ArrayRefs(val) => val.len(),
            _ => panic!("object is not a array"),
        }
    }
    // reflection
    pub fn get_ref_var(
        &self,
        name: &String,
        descriptor: &String,
    ) -> Option<Rc<RefCell<Object<'a>>>> {
        let field;
        {
            let rc = self.class.clone().upgrade().unwrap();
            field = rc.borrow().get_field(name, descriptor, false);
        }
        if let ObjectData::Field(f_var) = &self.data {
            return f_var.get_ref(field.borrow().slot_id as usize);
        } else {
            panic!("it's a array class, which have no field in object");
        }
    }
    pub fn set_ref_var(
        &mut self,
        name: &String,
        descriptor: &String,
        data: Option<Rc<RefCell<Object<'a>>>>,
    ) {
        let field;
        {
            let rc = self.class.clone().upgrade().unwrap();
            field = rc.borrow().get_field(name, descriptor, false);
        }
        if let ObjectData::Field(f_var) = &mut self.data {
            f_var.set_ref(field.borrow().slot_id as usize, data);
        } else {
            panic!("it's a array class, which have no field in object");
        }
    }
    pub fn new_array(class: Weak<RefCell<Class<'a>>>, count: usize) -> Object<'a> {
        let name;
        {
            name = class.clone().upgrade().unwrap().borrow().name.to_owned();
        }
        Object {
            class: class,
            data: match name.as_str() {
                "[Z" | "[B" => {
                    let mut v: Vec<Byte> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0);
                    }
                    ObjectData::ArrayBytes(v)
                }
                "[C" => {
                    let mut v: Vec<Char> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0);
                    }
                    ObjectData::ArrayChars(v)
                }
                "[S" => {
                    let mut v: Vec<Short> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0);
                    }
                    ObjectData::ArrayShorts(v)
                }
                "[I" => {
                    let mut v: Vec<Int> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0);
                    }
                    ObjectData::ArrayInts(v)
                }
                "[J" => {
                    let mut v: Vec<Long> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0);
                    }
                    ObjectData::ArrayLongs(v)
                }
                "[F" => {
                    let mut v: Vec<Float> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0.0);
                    }
                    ObjectData::ArrayFloats(v)
                }
                "[D" => {
                    let mut v: Vec<Double> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(0.0);
                    }
                    ObjectData::ArrayDoubles(v)
                }
                _ => {
                    let mut v: Vec<Ref> = Vec::with_capacity(count);
                    for _ in 0..count {
                        v.push(Rc::new(RefCell::new(Object {
                            class: Weak::new(),
                            data: ObjectData::Nil,
                        })));
                    }
                    ObjectData::ArrayRefs(v)
                }
            },
        }
    }
}
