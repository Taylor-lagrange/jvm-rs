use crate::runtime::heap::class_loader::ClassLoader;
use crate::runtime::thread::Frame;

// public final native Class<?> getClass();
// ()Ljava/lang/Class;
pub fn get_class_impl(frame: &mut Frame) {
    let this = frame.local_vars.get_ref(0).unwrap();
    let class = this.borrow().class.upgrade().unwrap();
    let rc = class.borrow();
    let j_class = rc.j_class.clone().unwrap();
    frame.operand_stack.push_ref(Some(j_class));
}

// public native int hashCode();
// ()I
pub fn hash_code_impl(frame: &mut Frame) {
    let this = frame.local_vars.get_ref(0).unwrap();
    let hash_str = format!("{:p}", this.as_ptr()).replace("0x", "");
    let hash = u64::from_str_radix(hash_str.as_str(), 16);
    frame.operand_stack.push_int(hash.unwrap() as i32);
}

// protected native Object clone() throws CloneNotSupportedException;
// ()Ljava/lang/Object;
pub fn clone_impl(frame: &mut Frame) {
    let this = frame.local_vars.get_ref(0).unwrap();
    let is_cloneable;
    {
        let class = this.borrow().class.clone().upgrade().unwrap();
        let loader = class.borrow().loader.clone();
        let cloneable = ClassLoader::load_class(loader, &"java/lang/Cloneable".to_string());
        let this_class = this.borrow().class.upgrade().unwrap();
        is_cloneable = this_class.borrow().is_implements(cloneable);
    }
    if !is_cloneable {
        panic!("java.lang.CloneNotSupportedException");
    }
    frame.operand_stack.push_ref(Some(this.clone()));
}
