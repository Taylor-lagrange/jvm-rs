use super::class_loader::*;
use super::object::*;
use std::cell::RefCell;
// use std::collections::HashMap;
use std::rc::{Rc, Weak};

// lazy_static! {
//   pub static ref INTERNED_STRINGS: RefCell<HashMap<String, Rc<RefCell<Object<'static>>>>> =
//     RefCell::new(HashMap::new());
// }

// TODO: string pool need to share object with static variable, so the lifetime of interned string need to be static
// but the reference store in the Object can't be static,so we have a conflict.
// in this way "abc" == "abc"  is false, because java compare two string by it's underlying object
// this to string are independent object, so to object are not equal

pub fn j_string<'a>(
    loader: Weak<RefCell<ClassLoader<'a>>>,
    r_string: &String,
) -> Rc<RefCell<Object<'a>>> {
    // let interned_string = INTERNED_STRINGS.borrow_mut();
    // if interned_string.contains_key(r_string) {
    //   return interned_string.get(r_string).unwrap().clone();
    // }
    let j_chars = Rc::new(RefCell::new(Object {
        class: ClassLoader::load_class(loader.clone(), &"[C".to_string()),
        data: ObjectData::ArrayChars(string_to_utf16(r_string)),
        extra: ObjectExtra::Nil,
    }));
    let j_str_class = ClassLoader::load_class(loader, &"java/lang/String".to_string())
        .upgrade()
        .unwrap();
    let j_str = Object::new_object(&j_str_class);
    j_str
        .borrow_mut()
        .set_ref_var(&"value".to_string(), &"[C".to_string(), Some(j_chars));
    j_str
}

pub fn rs_string(j_str: &Rc<RefCell<Object>>) -> String {
    let char_arr = j_str
        .borrow()
        .get_ref_var(&"value".to_string(), &"[C".to_string());
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
