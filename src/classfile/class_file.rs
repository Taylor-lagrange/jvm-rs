use super::class_reader::*;

struct ClassFile {
  // magic: u32,
  minor_version: u16,
  major_version: u16,
  // constant_pool:ConstantPool,
  access_flags: u16,
  // class⽂件存储的类名类似完全限定名，但是把点换成了斜线，Java语⾔规范把这种名字叫作二进制名（binarynames）
  this_class: u16,
  super_class: u16,
  interfaces: Vec<u16>,
  // fields: Vec<MemberInfo>,
  // methods Vec<MemberInfo>,
  // attributes Vec<AttributeInfo>,
}

impl ClassFile {
  fn parse(class_data: Vec<u8>) -> Result<ClassFile, std::string::ParseError> {
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
    let access = reader.read_u16();
    let this_class = reader.read_u16();
    let super_class = reader.read_u16();
    let interfaces = reader.read_u16s();
    Ok(ClassFile {
      minor_version: minor_version,
      major_version: major_version,
      access_flags: access,
      this_class: this_class,
      super_class: super_class,
      interfaces: interfaces,
    })
  }
  // fn read_and_check_magic()
}
