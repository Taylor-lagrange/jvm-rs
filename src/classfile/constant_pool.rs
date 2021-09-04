use super::class_reader::*;
use byteorder::{BigEndian, ByteOrder};

pub const CONSTANT_CLASS: u8 = 7;
pub const CONSTANT_FIELD_REF: u8 = 9;
pub const CONSTANT_METHOD_REF: u8 = 10;
pub const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
pub const CONSTANT_STRING: u8 = 8;
pub const CONSTANT_INTEGER: u8 = 3;
pub const CONSTANT_FLOAT: u8 = 4;
pub const CONSTANT_LONG: u8 = 5;
pub const CONSTANT_DOUBLE: u8 = 6;
pub const CONSTANT_NAME_AND_TYPE: u8 = 12;
pub const CONSTANT_UTF8: u8 = 1;
// pub const CONSTANT_METHOD_HANDLE: u8 = 15;
// pub const CONSTANT_METHOD_TYPE: u8 = 16;
// pub const CONSTANT_INVOKE_DYNAMIC: u8 = 18;

#[derive(Clone, Debug)]
pub enum ConstantInfo {
  Class {
    name_index: usize,
  },
  Fieldref {
    class_index: usize,
    name_and_type_index: usize,
  },
  Methodref {
    class_index: usize,
    name_and_type_index: usize,
  },
  InterfaceMethodref {
    class_index: usize,
    name_and_type_index: usize,
  },
  String {
    string_index: usize,
  },
  Integer {
    val: i32,
  },
  Float {
    val: f32,
  },
  Long {
    val: i64,
  },
  Double {
    val: f64,
  },
  NameAndType {
    name_index: usize,
    descriptor_index: usize,
  },
  Utf8 {
    str: String,
  },
  Nil, //for empty
}

pub struct ConstantPool {
  pub pool: Vec<ConstantInfo>,
  pub pool_count: usize,
}

impl ConstantPool {
  pub fn new(reader: &mut ClassReader) -> ConstantPool {
    let count = reader.read_u16() as usize;
    let mut cp = ConstantPool {
      pool_count: count,
      pool: Vec::with_capacity(count),
    };
    for _ in 0..count {
      cp.pool.push(ConstantInfo::Nil);
    }
    let mut i = 1;
    while i < cp.pool_count {
      cp.pool[i] = ConstantInfo::get_constant_info(reader);
      // long double占两个位置
      match cp.pool[i] {
        ConstantInfo::Long { .. } => {
          i += 1;
        }
        ConstantInfo::Double { .. } => {
          i += 1;
        }
        _ => {}
      };
      i += 1;
    }
    cp
  }
  pub fn get_constant_info(&self, index: usize) -> &ConstantInfo {
    &self.pool[index]
  }
  pub fn get_name_and_type(&self, index: usize) -> (&String, &String) {
    if let ConstantInfo::NameAndType {
      name_index,
      descriptor_index,
    } = self.get_constant_info(index)
    {
      return (self.get_utf8(*name_index), self.get_utf8(*descriptor_index));
    }
    panic!("no such utf8 constant!")
  }
  pub fn get_class_name(&self, index: usize) -> &String {
    if let ConstantInfo::Class { name_index } = self.get_constant_info(index) {
      return self.get_utf8(*name_index);
    }
    panic!("no such utf8 constant!")
  }
  pub fn get_utf8(&self, index: usize) -> &String {
    if let ConstantInfo::Utf8 { str } = self.get_constant_info(index) {
      return &str;
    }
    panic!("no such utf8 constant!")
  }
}

impl ConstantInfo {
  fn get_constant_info(reader: &mut ClassReader) -> ConstantInfo {
    let tag = reader.read_u8();
    match tag {
      CONSTANT_CLASS => ConstantInfo::Class {
        name_index: reader.read_u16() as usize,
      },
      CONSTANT_FIELD_REF => ConstantInfo::Fieldref {
        class_index: reader.read_u16() as usize,
        name_and_type_index: reader.read_u16() as usize,
      },
      CONSTANT_METHOD_REF => ConstantInfo::Methodref {
        class_index: reader.read_u16() as usize,
        name_and_type_index: reader.read_u16() as usize,
      },
      CONSTANT_INTERFACE_METHOD_REF => ConstantInfo::InterfaceMethodref {
        class_index: reader.read_u16() as usize,
        name_and_type_index: reader.read_u16() as usize,
      },
      CONSTANT_STRING => ConstantInfo::String {
        string_index: reader.read_u16() as usize,
      },
      CONSTANT_INTEGER => ConstantInfo::Integer {
        val: reader.read_u32() as i32,
      },
      CONSTANT_FLOAT => ConstantInfo::Float {
        val: BigEndian::read_f32(&reader.read_bytes(&4)),
      },
      CONSTANT_LONG => ConstantInfo::Long {
        val: BigEndian::read_i64(&reader.read_bytes(&8)),
      },
      CONSTANT_DOUBLE => ConstantInfo::Double {
        val: BigEndian::read_f64(&reader.read_bytes(&8)),
      },
      CONSTANT_NAME_AND_TYPE => ConstantInfo::NameAndType {
        name_index: reader.read_u16() as usize,
        descriptor_index: reader.read_u16() as usize,
      },
      CONSTANT_UTF8 => {
        let len = reader.read_u16();
        let mut str = String::new();
        for _ in 0..len {
          str.push(reader.read_u8() as char);
        }
        ConstantInfo::Utf8 { str: str }
      }
      _ => {
        panic!("unsupport const type")
      } // 以下三个指令是 Java SE 7才添加到 class 文件中，目的是支持新增的 invokedynamic 指令, 这里不使用，有了直接panic
        // CONSTANT_METHOD_HANDLE => {}
        // CONSTANT_METHOD_TYPE => {}
        // CONSTANT_INVOKE_DYNAMIC => {}
    }
  }
}
