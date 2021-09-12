use std::mem::transmute;
use std::rc::Rc;

pub struct BytecodeReader {
    code: Rc<Vec<u8>>,
    pub pc: usize,
}

impl BytecodeReader {
    pub fn new(code: Rc<Vec<u8>>, pc: usize) -> BytecodeReader {
        BytecodeReader { code: code, pc: pc }
    }
    pub fn reset_pc(&mut self, pc: usize) {
        self.pc = pc;
    }
    pub fn reset(&mut self, code: Rc<Vec<u8>>, pc: usize) {
        self.code = code;
        self.pc = pc;
    }
    pub fn read_u8(&mut self) -> u8 {
        self.pc += 1;
        self.code[self.pc - 1]
    }
    pub fn read_i8(&mut self) -> i8 {
        self.pc += 1;
        self.code[self.pc - 1] as i8
    }
    pub fn read_u16(&mut self) -> u16 {
        let b1 = self.read_u8() as u16;
        let b2 = self.read_u8() as u16;
        (b1 << 8) | b2
    }
    pub fn read_i16(&mut self) -> i16 {
        unsafe { transmute(self.read_u16()) }
    }
    pub fn read_i32(&mut self) -> i32 {
        let b1 = self.read_u8() as i32;
        let b2 = self.read_u8() as i32;
        let b3 = self.read_u8() as i32;
        let b4 = self.read_u8() as i32;
        (b1 << 24) | (b2 << 16) | (b3 << 8) | b4
    }
    pub fn read_i32s(&mut self, n: usize) -> Vec<i32> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.read_i32());
        }
        v
    }
    pub fn skip_padding(&mut self) {
        while self.pc % 4 != 0 {
            self.read_i8();
        }
    }
}
