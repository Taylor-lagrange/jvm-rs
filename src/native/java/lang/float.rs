use crate::runtime::thread::Frame;
use std::mem::transmute;

// public static native int floatToRawIntBits(float value);
// (F)I
pub fn float_to_raw_int_bits_impl(frame: &mut Frame) {
    let val = frame.local_vars.get_float(0);
    let data: i32 = unsafe { transmute(val) };
    frame.operand_stack.push_int(data);
}

// public static native float intBitsToFloat(int bits);
// (I)F
pub fn int_bits_to_float_impl(frame: &mut Frame) {
    let val = frame.local_vars.get_int(0);
    let data: f32 = unsafe { transmute(val) };
    frame.operand_stack.push_float(data);
}
