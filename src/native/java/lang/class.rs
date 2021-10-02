use crate::runtime::heap::class::Class;
use crate::runtime::heap::class_loader::ClassLoader;
use crate::runtime::heap::object::ObjectExtra;
use crate::runtime::heap::string_pool::{j_string, rs_string};
use crate::runtime::thread::Frame;
use log::debug;

// static native Class<?> getPrimitiveClass(String name);
// (Ljava/lang/String;)Ljava/lang/Class;
pub fn get_primitive_class_impl(frame: &mut Frame) {
    let name_obj = frame.local_vars.get_ref(0).unwrap();
    let name = rs_string(&name_obj);
    let j_class;
    {
        let rc_class = frame
            .method
            .borrow()
            .class_member
            .class
            .clone()
            .upgrade()
            .unwrap();
        let class = ClassLoader::load_class(rc_class.borrow().loader.clone(), &name)
            .upgrade()
            .unwrap();
        j_class = class.borrow().j_class.clone();
    }
    frame.operand_stack.push_ref(j_class);
}

// private native String getName0();
// ()Ljava/lang/String;
// 该方法调用方法 类.class.getName() 会返回这个类的类名
pub fn get_name0_impl(frame: &mut Frame) {
    let this = frame.local_vars.get_ref(0).unwrap();
    let extra = this.borrow().extra.clone();
    if let ObjectExtra::Class(c) = extra {
        let class = c.upgrade().unwrap();
        let name = class.borrow().java_name();
        let name_obj = j_string(class.borrow().loader.clone(), &name);
        frame.operand_stack.push_ref(Some(name_obj));
        return;
    }
    panic!("can't get name of this class")
}

// private static native boolean desiredAssertionStatus0(Class<?> clazz);
// (Ljava/lang/Class;)Z
pub fn desired_assertion_status0_impl(frame: &mut Frame) {
    // TODO
    frame.operand_stack.push_boolean(false);
}

// public native boolean isInterface();
// ()Z
// fn is_interface(frame: &mut Frame) {
//     let this = frame.local_vars.get_ref(0).unwrap();
//     let extra = this.borrow().extra.clone();
//     if let ObjectExtra::Class(c) = extra {
//         let class = c.upgrade().unwrap();
//         frame
//             .operand_stack
//             .push_boolean(class.borrow().is_interface());
//     }
// }

// public native boolean isPrimitive();
// ()Z
// fn is_primitive(frame: &mut Frame) {
//     let this = frame.local_vars.get_ref(0).unwrap();
//     let extra = this.borrow().extra.clone();
//     if let ObjectExtra::Class(c) = extra {
//         let class = c.upgrade().unwrap();
//         frame
//             .operand_stack
//             .push_boolean(class.borrow().is_primitive());
//     }
// }
