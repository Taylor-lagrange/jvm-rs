use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::class_init::*;
use crate::instructions::base::instruction::*;
use crate::instructions::base::method_invoke::*;
use crate::runtime::heap::class::*;
use crate::runtime::heap::constant_pool::*;
use crate::runtime::thread::*;

pub struct INVOKE_STATIC {}

impl Index16Instruction for INVOKE_STATIC {}

// invokestatic    用来调用静态方法
// invokespecial   用来调用无需动态绑定的实例方法，如构造函数、私有方法、用super关键字调到的超类方法   这些情况调用谁是确定的
// invokeinterface 针对接口的动态绑定
// invokevirtual   else 类的继承层次固定，可以使用 vtable 加速查找

// 之所应单独定义 invokeinterface 指令，是因为通过这个指令调用时的this指针可以指向当前所有实现了该接口的类示例，没法使用 vtable 加速查找

impl Instruction for INVOKE_STATIC {
  fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
    let index = self.fetch_operands(reader, frame);
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
    if let ConstantInfoRunTime::Methodref(mut refs) = info {
      let method = refs.resolve_method();
      let class = refs.member_ref.sym_ref.resolved_class();
      let rc = method.upgrade().unwrap();
      {
        let method_instance = rc.borrow();
        if !method_instance.class_member.is_static() {
          panic!("java.lang.IncompatibleClassChangeError");
        }
        if !Class::init_started(&class) {
          frame.revert_pc();
          init_class(frame.thread.clone(), class.clone());
          return;
        }
      }
      invoke_method(frame, rc);
    }
  }
}
