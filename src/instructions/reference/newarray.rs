use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::class_init::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::class_loader::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

const AT_BOOLEAN: u8 = 4;
const AT_CHAR: u8 = 5;
const AT_FLOAT: u8 = 6;
const AT_DOUBLE: u8 = 7;
const AT_BYTE: u8 = 8;
const AT_SHORT: u8 = 9;
const AT_INT: u8 = 10;
const AT_LONG: u8 = 11;

pub struct NEW_ARRAY {}

impl Index8Instruction for NEW_ARRAY {}

impl Instruction for NEW_ARRAY {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let atype = self.fetch_operands(reader, frame);
        let count = frame.operand_stack.pop_int();
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException");
        }
        let arry_class;
        {
            let class = frame
                .method
                .borrow_mut()
                .class_member
                .class
                .clone()
                .upgrade()
                .unwrap();
            arry_class = get_primitive_array_class(class.borrow_mut().loader.clone(), atype as u8);
        }
        let obj = Object::new_array(arry_class, count as usize);
        frame
            .operand_stack
            .push_ref(Some(Rc::new(RefCell::new(obj))));
    }
}

fn get_primitive_array_class(
    loader: Weak<RefCell<ClassLoader>>,
    atype: u8,
) -> Weak<RefCell<Class>> {
    match atype {
        AT_BOOLEAN => ClassLoader::load_class(loader, &"[Z".to_string()),
        AT_CHAR => ClassLoader::load_class(loader, &"[C".to_string()),
        AT_FLOAT => ClassLoader::load_class(loader, &"[F".to_string()),
        AT_DOUBLE => ClassLoader::load_class(loader, &"[D".to_string()),
        AT_BYTE => ClassLoader::load_class(loader, &"[B".to_string()),
        AT_SHORT => ClassLoader::load_class(loader, &"[S".to_string()),
        AT_INT => ClassLoader::load_class(loader, &"[I".to_string()),
        AT_LONG => ClassLoader::load_class(loader, &"[J".to_string()),
        _ => panic!("Invalid atype!"),
    }
}
