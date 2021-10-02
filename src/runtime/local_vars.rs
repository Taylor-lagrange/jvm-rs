use super::heap::object::*;
use super::operand_stack::*;
use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;

pub type StaticFinalVar<'a> = LocalVars<'a>;
pub type FieldVar<'a> = LocalVars<'a>;

#[derive(Default, Clone)]
pub struct LocalVars<'a>(Vec<Slot<'a>>);

impl<'a> LocalVars<'a> {
    pub fn new(max_locals: usize) -> LocalVars<'a> {
        let mut v = Vec::new();
        for _ in 0..max_locals {
            v.push(Slot::Nil);
        }
        LocalVars(v)
    }
    pub fn set_int(&mut self, index: usize, val: i32) {
        self.0[index] = Slot::Num(val)
    }
    // 因为 go 语言里他实现 jvm 用的是一个结构体，存储我这定义枚举里的所有元素，go的结构体会被默认初始化
    // 比如一个 int，就算没初始化读出来的也是个 0 ，这和 Java 里的变量初始化机制非常相似
    // 但我用了 rust 的枚举，没初始化过的 int 变量在这是 Nil ，一读就报错。
    // 所以增加一个机制对于非 ref 的数字默认会读出 0
    pub fn get_int(&self, index: usize) -> i32 {
        match self.0[index] {
            Slot::Num(num) => num,
            Slot::RefObject(_) => panic!("LocalVars get number failed!"),
            Slot::Nil => 0,
        }
    }
    pub fn set_float(&mut self, index: usize, val: f32) {
        self.0[index] = Slot::Num(val.to_bits() as i32)
    }
    pub fn get_float(&self, index: usize) -> f32 {
        match self.0[index] {
            Slot::Num(num) => unsafe { transmute(num) },
            Slot::RefObject(_) => panic!("LocalVars get number failed!"),
            Slot::Nil => 0.0,
        }
    }
    pub fn set_long(&mut self, index: usize, val: i64) {
        self.0[index] = Slot::Num(val as i32);
        self.0[index + 1] = Slot::Num((val >> 32) as i32);
    }
    pub fn get_long(&self, index: usize) -> i64 {
        let low: u32 = match self.0[index] {
            Slot::Num(num) => num as u32,
            Slot::Nil => 0,
            Slot::RefObject(_) => panic!("LocalVars get number failed!"),
        };
        let high: u32 = match self.0[index + 1] {
            Slot::Num(num) => num as u32,
            Slot::Nil => 0,
            Slot::RefObject(_) => panic!("LocalVars get number failed!"),
        };
        (((high as u64) << 32) | (low as u64)) as i64
    }
    pub fn set_double(&mut self, index: usize, val: f64) {
        let data: i64 = unsafe { transmute(val) };
        self.set_long(index, data)
    }
    pub fn get_double(&self, index: usize) -> f64 {
        let data = self.get_long(index);
        unsafe { transmute(data) }
    }
    pub fn set_ref(&mut self, index: usize, ref_object: Option<Rc<RefCell<Object<'a>>>>) {
        self.0[index] = Slot::RefObject(ref_object);
    }
    pub fn get_ref(&self, index: usize) -> Option<Rc<RefCell<Object<'a>>>> {
        match &self.0[index] {
            Slot::RefObject(object) => {
                return if let Some(obj) = object {
                    Some(obj.clone())
                } else {
                    None
                }
            }
            Slot::Nil => return None,
            Slot::Num(..) => panic!("LocalVars get ref failed!"),
        }
    }
    pub fn set_slot(&mut self, index: usize, slot: Slot<'a>) {
        self.0[index] = slot;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_local_vars() {
        let mut vars = LocalVars::new(10);
        let pi: f32 = 3.1415926;
        let e: f64 = 2.71828182845;
        vars.set_int(0, 100);
        vars.set_int(1, -100);
        vars.set_long(2, 2997924580);
        vars.set_long(4, -2997924580);
        vars.set_float(6, pi);
        vars.set_double(7, e);

        assert_eq!(vars.get_int(0), 100);
        assert_eq!(vars.get_int(1), -100);
        assert_eq!(vars.get_long(2), 2997924580);
        assert_eq!(vars.get_long(4), -2997924580);
        assert_eq!(vars.get_float(6), pi);
        assert_eq!(vars.get_double(7), e);
    }
}
