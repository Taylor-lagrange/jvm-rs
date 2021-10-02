use crate::instructions::base::method_invoke::*;
use crate::runtime::heap::class_loader::ClassLoader;
use crate::runtime::heap::string_pool::j_string;
use crate::runtime::thread::Frame;

// private static native void initialize();
// ()V
// hack: just make VM.savedProps nonempty
pub fn initialize_impl(frame: &mut Frame) {
    let vm_class = frame
        .method
        .borrow()
        .class_member
        .class
        .clone()
        .upgrade()
        .unwrap();
    let saved_props = vm_class.borrow().get_ref_var(
        &"savedProps".to_string(),
        &"Ljava/util/Properties;".to_string(),
    );
    let key = j_string(vm_class.borrow().loader.clone(), &"foo".to_string());
    let val = j_string(vm_class.borrow().loader.clone(), &"bar".to_string());
    frame.operand_stack.push_ref(saved_props);
    frame.operand_stack.push_ref(Some(key));
    frame.operand_stack.push_ref(Some(val));

    let props_class = ClassLoader::load_class(
        vm_class.borrow().loader.clone(),
        &"java/util/Properties".to_string(),
    )
    .upgrade()
    .unwrap();
    let set_prop_method = props_class.borrow().get_instance_method(
        &"setProperty".to_string(),
        &"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;".to_string(),
    );
    invoke_method(frame, set_prop_method);
}
