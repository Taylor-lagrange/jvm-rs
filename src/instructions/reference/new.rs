use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::class_init::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::heap::object::*;
use crate::runtime::thread::*;

pub struct NEW {}

impl Index16Instruction for NEW {}

impl Instruction for NEW {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let info;
        {
            let index = self.fetch_operands(reader, frame);
            let rc = frame.method.borrow_mut().class_member.class.clone();
            let pool_rc = rc
                .upgrade()
                .unwrap()
                .borrow_mut()
                .constant_pool
                .clone()
                .unwrap();
            let mut cp = pool_rc.borrow_mut();
            info = cp.get_constant_info(index).clone();
        }
        if let ConstantInfoRunTime::Class(mut refs) = info {
            let class = refs.sym_ref.resolved_class();
            if !Class::init_started(&class) {
                frame.revert_pc();
                init_class(frame.thread.clone(), class.clone());
                return;
            }
            let rc = class.clone().upgrade().unwrap();
            let class_instance = rc.borrow();
            if class_instance.is_interface() || class_instance.is_abstract() {
                panic!("java.lang.InstantiationError");
            }
            let ref_obj = Object::new_object(&class.upgrade().unwrap());
            frame.operand_stack.push_ref(Some(ref_obj));
        }
    }
}
