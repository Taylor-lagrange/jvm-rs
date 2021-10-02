use crate::native::java::lang::class::{
    desired_assertion_status0_impl, get_name0_impl, get_primitive_class_impl,
};
use crate::native::java::lang::double::{double_to_raw_long_bits_impl, long_bits_to_double_impl};
use crate::native::java::lang::float::{float_to_raw_int_bits_impl, int_bits_to_float_impl};
use crate::native::java::lang::object::{clone_impl, get_class_impl, hash_code_impl};
use crate::native::java::lang::string::intern_impl;
use crate::native::java::lang::system::arraycopy_impl;
use crate::native::java::lang::throwable::fill_in_stack_trace_impl;
use crate::native::sun::misc::vm::initialize_impl;
use crate::runtime::thread::*;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub type NativeMethod = Arc<fn(frame: &mut Frame)>;

fn empty_native_method_impl(_frame: &mut Frame) {
    //do nothing
}

fn invalid_native_method_impl(_frame: &mut Frame) {
    //do nothing just panic
    panic!("invalid native method");
}

lazy_static! {
    pub static ref INVALID_NATIVE_METHOD: NativeMethod = Arc::new(invalid_native_method_impl);
    pub static ref EMPTY_NATIVE_METHOD: NativeMethod = Arc::new(empty_native_method_impl);
    static ref REGISTERY: RwLock<HashMap<String, NativeMethod>> = RwLock::new(HashMap::new());
}

pub fn native_method_register() {
    register(
        "java/lang/Class",
        "getPrimitiveClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        Arc::new(get_primitive_class_impl),
    );
    register(
        "java/lang/Class",
        "getName0",
        "()Ljava/lang/String;",
        Arc::new(get_name0_impl),
    );
    register(
        "java/lang/Class",
        "desiredAssertionStatus0",
        "(Ljava/lang/Class;)Z",
        Arc::new(desired_assertion_status0_impl),
    );
    register(
        "java/lang/Double",
        "doubleToRawLongBits",
        "(D)J",
        Arc::new(double_to_raw_long_bits_impl),
    );
    register(
        "java/lang/Double",
        "longBitsToDouble",
        "(J)D",
        Arc::new(long_bits_to_double_impl),
    );
    register(
        "java/lang/Float",
        "floatToRawIntBits",
        "(F)I",
        Arc::new(float_to_raw_int_bits_impl),
    );
    register(
        "java/lang/Float",
        "intBitsToFloat",
        "(I)F",
        Arc::new(int_bits_to_float_impl),
    );
    register(
        "java/lang/Object",
        "getClass",
        "()Ljava/lang/Class;",
        Arc::new(get_class_impl),
    );
    register(
        "java/lang/Object",
        "hashCode",
        "()I",
        Arc::new(hash_code_impl),
    );
    register(
        "java/lang/Object",
        "clone",
        "()Ljava/lang/Object;",
        Arc::new(clone_impl),
    );
    register(
        "java/lang/String",
        "intern",
        "()Ljava/lang/String;",
        Arc::new(intern_impl),
    );
    register(
        "java/lang/System",
        "arraycopy",
        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
        Arc::new(arraycopy_impl),
    );
    register(
        "sun/misc/VM",
        "initialize",
        "()V",
        Arc::new(initialize_impl),
    );
    register(
        "java/lang/Throwable",
        "fillInStackTrace",
        "(I)Ljava/lang/Throwable;",
        Arc::new(fill_in_stack_trace_impl),
    );
}

pub fn register(
    class_name: &str,
    method_name: &str,
    method_descriptor: &str,
    method: NativeMethod,
) {
    let key = format!("{}~{}~{}", class_name, method_name, method_descriptor);
    debug!("register method {}", key);
    REGISTERY.write().unwrap().insert(key, method);
}

pub fn find_native_method(
    class_name: &String,
    method_name: &String,
    method_descriptor: &String,
) -> NativeMethod {
    let key = format!("{}~{}~{}", class_name, method_name, method_descriptor);
    // 好多类在初始化的时候都会调用一个叫 registerNatives 的方法注册本地方法
    // 但是由于我实现的本地方法有限，我选择在启动 jvm 的时候直接把我写的方法全部注册上，所以registerNatives总是会返回一个空方法
    debug!("method {} be invoked", key);
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
