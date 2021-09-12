use crate::runtime::thread::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub type NativeMethod = Arc<fn(frame: &mut Frame)>;

fn empty_native_method_impl(_frame: &mut Frame) {
    //do nothing
}

fn invalid_native_method_impl(_frame: &mut Frame) {
    //do nothing
}

lazy_static! {
    pub static ref INVALID_NATIVE_METHOD: NativeMethod = Arc::new(invalid_native_method_impl);
    static ref EMPTY_NATIVE_METHOD: NativeMethod = Arc::new(empty_native_method_impl);
    static ref REGISTERY: RwLock<HashMap<String, NativeMethod>> = RwLock::new(HashMap::new());
}

pub fn register(
    class_name: &String,
    method_name: &String,
    method_descriptor: &String,
    method: NativeMethod,
) {
    let key = format!("{}~{}~{}", class_name, method_name, method_descriptor);
    REGISTERY.write().unwrap().insert(key, method);
}

pub fn find_native_method(
    class_name: &String,
    method_name: &String,
    method_descriptor: &String,
) -> NativeMethod {
    let key = format!("{}~{}~{}", class_name, method_name, method_descriptor);
    let method: Option<NativeMethod> = match REGISTERY.read().unwrap().get(&key) {
        Some(val) => Some((*val).clone()),
        None => None,
    };
    if method.is_some() {
        return method.unwrap();
    }
    if method_descriptor == "()V" && method_name == "registerNatives" {
        return EMPTY_NATIVE_METHOD.clone();
    }
    return INVALID_NATIVE_METHOD.clone();
}
