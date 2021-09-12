use super::heap::object::*;
use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;

#[derive(Clone)]
pub enum Slot<'a> {
    Num(i32),
    RefObject(Option<Rc<RefCell<Object<'a>>>>),
    Nil,
}

pub struct OperandStack<'a> {
    size: usize,
    slots: Vec<Slot<'a>>,
}

impl<'a> OperandStack<'a> {
    pub fn new(max_stack: usize) -> OperandStack<'a> {
        OperandStack {
            size: 0,
            slots: Vec::with_capacity(max_stack),
        }
    }
    pub fn push_int(&mut self, val: i32) {
        self.size += 1;
        self.slots.push(Slot::Num(val))
    }
    pub fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        if let Some(Slot::Num(num)) = self.slots.pop() {
            num
        } else {
            panic!("invalid: pop when empty stack")
        }
    }
    pub fn push_float(&mut self, val: f32) {
        let data = val.to_bits();
        self.size += 1;
        self.slots.push(Slot::Num(data as i32))
    }
    pub fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        if let Some(Slot::Num(num)) = self.slots.pop() {
            let data: f32 = unsafe { transmute(num) };
            data
        } else {
            panic!("invalid: pop when empty stack")
        }
    }
    pub fn push_long(&mut self, val: i64) {
        self.size += 2;
        self.slots.push(Slot::Num(val as i32));
        self.slots.push(Slot::Num((val >> 32) as i32))
    }
    pub fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        let low: u32;
        let high: u32;
        if let Some(Slot::Num(num)) = self.slots.pop() {
            high = num as u32;
        } else {
            panic!("invalid: pop when empty stack")
        }
        if let Some(Slot::Num(num)) = self.slots.pop() {
            low = num as u32;
        } else {
            panic!("invalid: pop when empty stack")
        }
        (((high as u64) << 32) | (low as u64)) as i64
    }
    pub fn push_double(&mut self, val: f64) {
        let data: i64 = unsafe { transmute(val) };
        self.push_long(data)
    }
    pub fn pop_double(&mut self) -> f64 {
        let data = self.pop_long();
        unsafe { transmute(data) }
    }
    pub fn push_ref(&mut self, ref_object: Option<Rc<RefCell<Object<'a>>>>) {
        self.size += 1;
        self.slots.push(Slot::RefObject(ref_object));
    }
    pub fn pop_ref(&mut self) -> Option<Rc<RefCell<Object<'a>>>> {
        if let Some(Slot::RefObject(object)) = self.slots.pop() {
            self.size -= 1;
            object
        } else {
            panic!("invalid: pop when empty stack")
        }
    }
    pub fn pop_slot(&mut self) -> Slot<'a> {
        if let Some(object) = self.slots.pop() {
            self.size -= 1;
            object
        } else {
            panic!("invalid: pop when empty stack")
        }
    }
    pub fn push_slot(&mut self, slot: Slot<'a>) {
        self.size += 1;
        self.slots.push(slot);
    }
    pub fn get_ref_from_top(&self, n: usize) -> &Slot<'a> {
        &self.slots[self.size - 1 - n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operand_stack() {
        let mut stack = OperandStack::new(10);
        let pi: f32 = 3.1415926;
        let e: f64 = 2.71828182845;
        stack.push_int(100);
        stack.push_int(-100);
        stack.push_long(2997924580);
        stack.push_long(-2997924580);
        stack.push_float(pi);
        stack.push_double(e);
        assert_eq!(stack.pop_double(), e);
        assert_eq!(stack.pop_float(), pi);
        assert_eq!(stack.pop_long(), -2997924580);
        assert_eq!(stack.pop_long(), 2997924580);
        assert_eq!(stack.pop_int(), -100);
        assert_eq!(stack.pop_int(), 100);
    }
}
