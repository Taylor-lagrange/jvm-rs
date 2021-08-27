use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct LDC {} // Push item from run-time constant pool
pub struct LDC_W {} // Push item from run-time constant pool (wide index)
pub struct LDC2_W {} // Push long or double from run-time constant pool (wide index)

impl Index8Instruction for LDC {}
impl Index16Instruction for LDC_W {}
impl Index16Instruction for LDC2_W {}

fn ldc(frame: &mut Frame, index: usize) {
  let rc = frame.method.borrow_mut().class_member.class.clone();
  let pool_rc = rc
    .upgrade()
    .unwrap()
    .borrow_mut()
    .constant_pool
    .clone()
    .unwrap();
  let mut cp = pool_rc.borrow_mut();
  match cp.get_constant_info(index) {
    ConstantInfoRunTime::Integer(val) => frame.operand_stack.push_int(*val),
    ConstantInfoRunTime::Float(val) => frame.operand_stack.push_float(*val),
    ConstantInfoRunTime::Long(val) => frame.operand_stack.push_long(*val),
    ConstantInfoRunTime::Double(val) => frame.operand_stack.push_double(*val),
    _ => panic!("todo"),
  }
}

impl Instruction for LDC {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    ldc(frame, index);
  }
}
impl Instruction for LDC_W {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    ldc(frame, index);
  }
}
impl Instruction for LDC2_W {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
    ldc(frame, index);
  }
}
