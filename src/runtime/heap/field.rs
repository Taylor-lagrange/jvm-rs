use super::access_flags::*;
use super::class::*;
use super::class_member::*;
use crate::classfile::attribute_info::*;
use crate::classfile::constant_pool::*;
use crate::classfile::member_info::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default, Clone)]
pub struct Field<'a> {
    pub class_member: ClassMember<'a>,
    pub const_value_index: u32,
    pub slot_id: u32,
}

impl<'a> Field<'a> {
    pub fn new_fields(
        class: Weak<RefCell<Class<'a>>>,
        pool: &ConstantPool,
        cf_fields: Vec<MemberInfo>,
    ) -> Vec<Rc<RefCell<Field<'a>>>> {
        let mut fields = Vec::with_capacity(cf_fields.len());
        for info in cf_fields.iter() {
            fields.push(Rc::new(RefCell::new(Field::new(class.clone(), pool, info))))
        }
        fields
    }
    fn new(
        class: Weak<RefCell<Class<'a>>>,
        pool: &ConstantPool,
        member_info: &MemberInfo,
    ) -> Field<'a> {
        let mut field = Field {
            class_member: ClassMember::new(pool, class, member_info),
            ..Default::default()
        };
        if let AttributeInfo::ConstantValue {
            constant_value_index,
        } = member_info.constant_value_attribute()
        {
            field.const_value_index = constant_value_index as u32;
        }
        field
    }
    pub fn is_volatile(&self) -> bool {
        self.class_member.access_flags & ACC_VOLATILE != 0
    }
    pub fn is_transient(&self) -> bool {
        self.class_member.access_flags & ACC_TRANSIENT != 0
    }
    pub fn is_enum(&self) -> bool {
        self.class_member.access_flags & ACC_ENUM != 0
    }
    pub fn is_long_or_double(&self) -> bool {
        self.class_member.descriptor == "J" || self.class_member.descriptor == "D"
    }
}
