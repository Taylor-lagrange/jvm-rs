use crate::runtime::thread::Frame;

// public native String intern();
// ()Ljava/lang/String;
pub fn intern_impl(frame: &mut Frame) {
    // TODO: because intern string unimplemented, so this native method just a fake method
    let this = frame.local_vars.get_ref(0).unwrap();
    frame.operand_stack.push_ref(Some(this));
}
