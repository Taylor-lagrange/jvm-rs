pub struct ClassReader {
    data: Vec<u8>,
    index: usize,
}

// isize 和 usize 两种整数类型是用来衡量数据大小的
// 它们的位长度取决于所运行的目标平台，如果是 32 位架构的处理器将使用 32 位位长度整型

impl ClassReader {
    pub fn new(v: Vec<u8>) -> ClassReader {
        ClassReader { data: v, index: 0 }
    }
    pub fn read_u8(&mut self) -> u8 {
        self.index += 1;
        self.data[self.index - 1]
    }
    pub fn read_u16(&mut self) -> u16 {
        self.index += 2;
        ((self.data[self.index - 2] as u16) << 8) | self.data[self.index - 1] as u16
    }
    pub fn read_u32(&mut self) -> u32 {
        self.index += 4;
        ((self.data[self.index - 4] as u32) << 24)
            | ((self.data[self.index - 3] as u32) << 16)
            | ((self.data[self.index - 2] as u32) << 8)
            | self.data[self.index - 1] as u32
    }
    pub fn read_u64(&mut self) -> u64 {
        self.index += 8;
        ((self.data[self.index - 8] as u64) << 56)
            | ((self.data[self.index - 7] as u64) << 48)
            | ((self.data[self.index - 6] as u64) << 40)
            | ((self.data[self.index - 5] as u64) << 32)
            | ((self.data[self.index - 4] as u64) << 24)
            | ((self.data[self.index - 3] as u64) << 16)
            | ((self.data[self.index - 2] as u64) << 8)
            | self.data[self.index - 1] as u64
    }
    // read_u16s 读取uint16表，表的大小由开头的uint16数据指出
    pub fn read_u16s(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut list = Vec::new();
        for _ in 0..n {
            list.push(self.read_u16());
        }
        list
    }
    pub fn read_bytes(&mut self, n: &usize) -> Vec<u8> {
        let mut li = Vec::new();
        for _ in 0..*n {
            li.push(self.read_u8());
        }
        li
    }
}
