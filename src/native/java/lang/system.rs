use crate::runtime::heap::object::Object;
use crate::runtime::thread::Frame;
use std::cell::RefCell;
use std::rc::Rc;

// public static native void arraycopy(Object src, int srcPos, Object dest, int destPos, int length)
// (Ljava/lang/Object;ILjava/lang/Object;II)V
pub fn arraycopy_impl(frame: &mut Frame) {
    let src_op = frame.local_vars.get_ref(0);
    let src_pos = frame.local_vars.get_int(1);
    let des_op = frame.local_vars.get_ref(2);
    let des_pos = frame.local_vars.get_int(3);
    let len = frame.local_vars.get_int(4);
    if src_op.is_none() || des_op.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let src = src_op.unwrap();
    let des = des_op.unwrap();
    if !check_array_copy(src.clone(), des.clone()) {
        panic!("java.lang.ArrayStoreException");
    }
    if src_pos < 0
        || des_pos < 0
        || len < 0
        || src_pos + len > src.borrow().array_length() as i32
        || des_pos + len > des.borrow().array_length() as i32
    {
        panic!("java.lang.IndexOutOfBoundsException");
    }
    Object::array_copy(src, des, src_pos as usize, des_pos as usize, len as usize);
}

fn check_array_copy(_src: Rc<RefCell<Object>>, _des: Rc<RefCell<Object>>) -> bool {
    //TODO: unimplemented

    // srcClass := src.Class()
    // destClass := dest.Class()
    //
    // if !srcClass.IsArray() || !destClass.IsArray() {
    //     return false
    // }
    // if srcClass.ComponentClass().IsPrimitive() ||
    //     destClass.ComponentClass().IsPrimitive() {
    //     return srcClass == destClass
    // }
    true
}
