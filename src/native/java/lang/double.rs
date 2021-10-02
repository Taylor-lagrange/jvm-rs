use crate::runtime::thread::Frame;
use std::mem::transmute;

// public static native long doubleToRawLongBits(double value);
// (D)J
pub fn double_to_raw_long_bits_impl(frame: &mut Frame) {
    let val = frame.local_vars.get_double(0);
    let data: i64 = unsafe { transmute(val) };
    frame.operand_stack.push_long(data);
}

// public static native double longBitsToDouble(long bits);
// (J)D
pub fn long_bits_to_double_impl(frame: &mut Frame) {
    let val = frame.local_vars.get_long(0);
    let data: f64 = unsafe { transmute(val) };
    frame.operand_stack.push_double(data);
}
