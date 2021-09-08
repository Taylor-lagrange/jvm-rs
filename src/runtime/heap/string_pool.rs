use super::object::*;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::{Rc};

// thread_local!(static INTERNED_STRINGS: RefCell<HashMap<String, Rc<Object<'static>>>> = RefCell::new(HashMap::new()));

// TODO: string pool need to share object, but my original design is not consider this situation.
// so all of my struct use to store object not warp by Rc, which lead a huge cost to implement string pool.
// besides, most object need modify, whcih need Rc<RefCell<>>, introduce a lot of run time borrow-checker for string pool is not worth

// pub fn j_string<'a>(loader: Weak<RefCell<ClassLoader<'a>>>, name: &String) -> Rc<Object<'a>> {
//   if INTERNED_STRINGS.contains_key(name) {
//     return INTERNED_STRINGS.get(name).unwrap().clone();
//   }

fn rs_string(j_str: &Object) -> String {
  let char_arr = j_str.get_ref_var(&"value".to_string(), &"[C".to_string());
  let char_obj = char_arr.expect("can't find value([]char) in current object!");
  let rc = char_obj.borrow();
  if let ObjectData::ArrayChars(char_array) = &rc.data {
    return utf16_to_string(char_array);
  } else {
    panic!("object's data is not a char array!");
  }
}

// rust string -> utf16
fn string_to_utf16(s: &String) -> Vec<u16> {
  s.encode_utf16().collect()
}

// // utf16 -> rust string
fn utf16_to_string(s: &Vec<u16>) -> String {
  String::from_utf16(s).unwrap()
}
