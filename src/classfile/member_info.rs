use super::attribute_info::*;
use super::class_reader::*;
use super::constant_pool::*;

#[derive(Clone)]
pub struct MemberInfo {
  pub access_flags: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes: Vec<AttributeInfo>,
}

impl MemberInfo {
  pub fn read_members(reader: &mut ClassReader, cp: &ConstantPool) -> Vec<MemberInfo> {
    let count = reader.read_u16();
    let mut v = Vec::new();
    for _ in 0..count {
      v.push(MemberInfo::read_member(reader, cp));
    }
    v
  }

  pub fn read_member(reader: &mut ClassReader, cp: &ConstantPool) -> MemberInfo {
    MemberInfo {
      access_flags: reader.read_u16(),
      name_index: reader.read_u16(),
      descriptor_index: reader.read_u16(),
      attributes: AttributeInfo::read_attributes(reader, cp),
    }
  }

  pub fn code_attribute(&self) -> AttributeInfo {
    for info in self.attributes.iter() {
      if let AttributeInfo::Code { .. } = info {
        return info.clone()
      }
    }
    panic!("no code attribute in member info");
  }
}
