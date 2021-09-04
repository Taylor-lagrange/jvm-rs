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
  pub fn get_int(&mut self, index: usize) -> i32 {
    if let Slot::Num(num) = self.0[index] {
      num
    } else {
      panic!("LocalVars get number failed!")
    }
  }
  pub fn set_float(&mut self, index: usize, val: f32) {
    self.0[index] = Slot::Num(val.to_bits() as i32)
  }
  pub fn get_float(&mut self, index: usize) -> f32 {
    if let Slot::Num(num) = self.0[index] {
      let data: f32 = unsafe { transmute(num) };
      data
    } else {
      panic!("LocalVars get number failed!")
    }
  }
  pub fn set_long(&mut self, index: usize, val: i64) {
    self.0[index] = Slot::Num(val as i32);
    self.0[index + 1] = Slot::Num((val >> 32) as i32);
  }
  pub fn get_long(&mut self, index: usize) -> i64 {
    let low: u32;
    let high: u32;
    if let Slot::Num(num) = self.0[index] {
      low = num as u32;
    } else {
      panic!("LocalVars get number failed!")
    }
    if let Slot::Num(num) = self.0[index + 1] {
      high = num as u32;
    } else {
      panic!("LocalVars get number failed!")
    }
    (((high as u64) << 32) | (low as u64)) as i64
  }
  pub fn set_double(&mut self, index: usize, val: f64) {
    let data: i64 = unsafe { transmute(val) };
    self.set_long(index, data)
  }
  pub fn get_double(&mut self, index: usize) -> f64 {
    let data = self.get_long(index);
    unsafe { transmute(data) }
  }
  pub fn set_ref(&mut self, index: usize, ref_object: Option<Rc<RefCell<Object<'a>>>>) {
    self.0[index] = Slot::RefObject(ref_object);
  }
  pub fn get_ref(&mut self, index: usize) -> Option<Rc<RefCell<Object<'a>>>> {
    match &self.0[index] {
      Slot::RefObject(object) => {
        if let Some(obj) = object {
          return Some(obj.clone());
        } else {
          return None;
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
