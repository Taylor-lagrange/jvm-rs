use super::attribute_info::*;
use super::class_reader::*;
use super::constant_pool::*;
use super::member_info::*;
use log::info;

pub struct ClassFile {
  // magic: u32,
  minor_version: u16,
  major_version: u16,
  constant_pool: ConstantPool,
  access_flags: u16,
  // class⽂件存储的类名类似完全限定名，但是把点换成了斜线，Java语⾔规范把这种名字叫作二进制名（binarynames）
  this_class: u16,
  super_class: u16,
  interfaces: Vec<u16>,
  fields: Vec<MemberInfo>,
  methods: Vec<MemberInfo>,
  attributes: Vec<AttributeInfo>,
}

impl ClassFile {
  pub fn parse(class_data: Vec<u8>) -> Result<ClassFile, std::string::ParseError> {
    let mut reader = ClassReader::new(class_data);
    let magic = reader.read_u32();
    // first check the magic number
    if magic != 0xCAFEBABE {
      panic!("java.lang.ClassFormatError: magic!")
    }
    let minor_version = reader.read_u16();
    let major_version = reader.read_u16();
    match major_version {
      46 | 47 | 48 | 49 | 50 | 51 | 52 => {
        if minor_version != 0 {
          panic!("java.lang.UnsupportedClassVersionError!")
        }
      }
      _ => {}
    };
    let pool = ConstantPool::new(&mut reader);
    let access = reader.read_u16();
    let this_class = reader.read_u16();
    let super_class = reader.read_u16();
    let interfaces = reader.read_u16s();
    let fields = MemberInfo::read_members(&mut reader, &pool);
    let methods = MemberInfo::read_members(&mut reader, &pool);
    let attributes = AttributeInfo::read_attributes(&mut reader, &pool);
    Ok(ClassFile {
      minor_version: minor_version,
      major_version: major_version,
      constant_pool: pool,
      access_flags: access,
      this_class: this_class,
      super_class: super_class,
      interfaces: interfaces,
      fields: fields,
      methods: methods,
      attributes: attributes,
    })
  }
  fn class_name(&self) -> &String {
    self.constant_pool.get_class_name(self.this_class as usize)
  }
  fn super_class_name(&self) -> String {
    if self.super_class > 0 {
      self
        .constant_pool
        .get_class_name(self.super_class as usize)
        .to_owned()
    } else {
      "".to_string()
    }
  }
  fn interface_names(&self) -> Vec<&String> {
    let mut v = Vec::new();
    for i in 0..self.interfaces.len() {
      v.push(
        self
          .constant_pool
          .get_class_name(self.interfaces[i] as usize),
      )
    }
    v
  }
  pub fn print_class_info(&self) {
    info!("version: {}.{}\n", self.major_version, self.minor_version);
    info!("constants count: {}\n", self.constant_pool.pool_count);
    info!("access flags: 0x{:X}\n", self.access_flags);
    info!("this class: {}\n", self.class_name());
    info!("super class: {}\n", self.super_class_name());
    info!("interfaces: {:#?}\n", self.interface_names());
    info!("fields count: {}\n", self.fields.len());
    let mut field_str = String::new();
    for e in self.fields.iter() {
      field_str += self.constant_pool.get_utf8(e.name_index as usize);
      field_str += " ";
    }
    info!(" {}\n", field_str);
    let mut method_str = String::new();
    info!("methods count: {}\n", self.methods.len());
    for e in self.methods.iter() {
      method_str += self.constant_pool.get_utf8(e.name_index as usize);
      method_str += " ";
    }
    info!(" {}\n", method_str);
  }
  pub fn get_main_method(&self) -> &MemberInfo {
    for info in self.methods.iter() {
      if self.constant_pool.get_utf8(info.name_index as usize) == "main"
        && self.constant_pool.get_utf8(info.descriptor_index as usize) == "([Ljava/lang/String;)V"
      {
        return info;
      }
    }
    panic!("no main method in class {}", self.class_name())
  }
}
