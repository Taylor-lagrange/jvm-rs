use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct CHECK_CAST {}

impl Index16Instruction for CHECK_CAST {}

impl Instruction for CHECK_CAST {
    fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
        let index = self.fetch_operands(reader, frame);
        let target_ref = frame.operand_stack.pop_ref();
        frame.operand_stack.push_ref(target_ref.clone());
        if target_ref.is_none() {
            return;
        }
        let info;
        {
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
            if !target_ref.unwrap().borrow().is_instance_of(class) {
                panic!("java.lang.ClassCastException");
            }
        }
    }
}
