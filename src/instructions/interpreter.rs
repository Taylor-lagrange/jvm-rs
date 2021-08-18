use super::factory::*;
use crate::classfile::attribute_info::*;
use crate::classfile::member_info::*;
use crate::instructions::base::branch::*;
use crate::instructions::base::bytecode_reader::*;
use crate::instructions::base::instruction::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

pub fn interpret(method_info: MemberInfo) {
  let code_attr = method_info.code_attribute();
  let mut thread = Thread::new();
  let mut ms: usize = 0;
  let mut ml: usize = 0;
  let mut c: Vec<u8> = Vec::new();
  if let AttributeInfo::Code {
    max_stack,
    max_locals,
    code,
    ..
  } = code_attr
  {
    ms = max_stack as usize;
    ml = max_locals as usize;
    c = code;
  }
  let frame = Thread::new_frame(thread.clone(), ml as usize, ms as usize);
  thread.borrow_mut().stack.push(frame);
  run(thread, c);
}

pub fn run(thread: Rc<RefCell<Thread>>, code: Vec<u8>) {
  let mut frame = thread.borrow_mut().stack.pop();
  let mut reader = BytecodeReader::new(code, 0);
  loop {
    // update reader and thread pc by frame pc
    let pc: i32 = frame.next_pc as i32;
    thread.borrow_mut().pc = pc;
    reader.reset_pc(pc as usize);
    // fetch opcode from reader
    let opcode = reader.read_u8();
    let mut inst = new_instruction(opcode);
    // update frame pc because read of the opcode
    frame.next_pc = reader.pc;
    (*inst).execute(&mut reader, &mut frame);
  }
}

/*

test code:

public class GaussTest {
  public static void main(String[] args) {
    int sum = 0;
    for (int i = 1; i <= 100; i++) {
      sum += i;
    }
    System.out.println(sum);
  }
}

public class GaussTest {
  public GaussTest();
    Code:
       0: aload_0
       1: invokespecial #1                  // Method java/lang/Object."<init>":()V
       4: return

  public static void main(java.lang.String[]);
    Code:
3              0: iconst_0
60             1: istore_1
4              2: iconst_1
61             3: istore_2
28             4: iload_2
16 100         5: bipush        100
163 0 13       7: if_icmpgt     20
27             10: iload_1
28             11: iload_2
96             12: iadd
60             13: istore_1
132 2 1        14: iinc          2, 1
167 255 243    17: goto          4
               20: getstatic     #7                  // Field java/lang/System.out:Ljava/io/PrintStream;
               23: iload_1
               24: invokevirtual #13                 // Method java/io/PrintStream.println:(I)V
               27: return
}

*/

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_interpreter() {
    let member_info = MemberInfo {
      access_flags: 0,
      name_index: 0,
      descriptor_index: 0,
      attributes: vec![AttributeInfo::Code {
        max_stack: 2,
        max_locals: 3,
        code: vec![
          3, 60, 4, 61, 28, 16, 100, 163, 0, 13, 27, 28, 96, 60, 132, 2, 1, 167, 255, 243, 178, 0,
          7, 27, 182, 0, 13, 177,
        ],
        exception_table: Vec::new(),
        attributes: Vec::new(),
      }],
    };
    let result = panic::catch_unwind(|| {
      interpret(member_info);
    });
    assert!(result.is_err());
  }
}
