use super::factory::*;
use crate::instructions::base::bytecode_reader::*;
use crate::runtime::heap::class_loader::*;
use crate::runtime::heap::method::*;
use crate::runtime::heap::object::*;
use crate::runtime::heap::string_pool::*;
use crate::runtime::thread::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn interpret(method: Rc<RefCell<Method>>, args: Vec<String>) {
    let loader;
    {
        let rc = method.borrow().class_member.class.clone();
        loader = rc.upgrade().unwrap().borrow().loader.clone();
    }
    let j_args = create_args_array(loader, args);
    let thread = Thread::new();
    let frame = Thread::new_frame(Rc::downgrade(&thread), method.clone());
    frame
        .borrow_mut()
        .local_vars
        .set_ref(0, Some(Rc::new(RefCell::new(j_args))));
    thread.borrow_mut().stack.push(frame);

    run(thread);
}

pub fn create_args_array(loader: Weak<RefCell<ClassLoader>>, args: Vec<String>) -> Object {
    let string_class = ClassLoader::load_class(loader.clone(), &"java/lang/String".to_string());
    let mut args_arr = Object::new_array(string_class, args.len());
    if let ObjectData::ArrayRefs(j_args) = &mut args_arr.data {
        for i in 0..args.len() {
            j_args[i] = j_string(loader.clone(), &args[i]);
        }
    }
    args_arr
}

pub fn run(thread: Rc<RefCell<Thread>>) {
    let empty_vec = Rc::new(Vec::new());
    let mut reader = BytecodeReader::new(empty_vec, 0);

    loop {
        let pc;
        let code;
        {
            let mut thread_instance = thread.borrow_mut();
            let top = thread_instance.stack.top();
            let frame = top.borrow();
            // update reader and thread pc by frame pc
            pc = frame.next_pc as i32;
            let method = frame.method.borrow();
            code = method.code.clone()
        }
        thread.borrow_mut().pc = pc;

        reader.reset(code, pc as usize);

        // fetch opcode from reader
        let opcode = reader.read_u8();
        let mut inst = new_instruction(opcode);
        // update frame pc because read of the opcode
        {
            let frame;
            {
                let mut thread_instance = thread.borrow_mut();
                frame = thread_instance.stack.top();
            }
            frame.borrow_mut().next_pc = reader.pc;
            let borrow_frame = &mut frame.borrow_mut();
            (*inst).execute(&mut reader, borrow_frame);
        }
        if thread.borrow_mut().stack.is_empty() {
            break;
        }
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

// #[cfg(test)]
// mod test {
//   use super::*;

//   #[test]
//   fn test_interpreter() {
//     let method =Method::new(class: Weak<RefCell<Class<'a>>>, pool: &ConstantPool, member_info: &MemberInfo)
//     let member_info = MemberInfo {
//       access_flags: 0,
//       name_index: 0,
//       descriptor_index: 0,
//       attributes: vec![AttributeInfo::Code {
//         max_stack: 2,
//         max_locals: 3,
//         code: vec![
//           3, 60, 4, 61, 28, 16, 100, 163, 0, 13, 27, 28, 96, 60, 132, 2, 1, 167, 255, 243, 178, 0,
//           7, 27, 182, 0, 13, 177,
//         ],
//         exception_table: Vec::new(),
//         attributes: Vec::new(),
//       }],
//     };
//     let result = panic::catch_unwind(|| {
//       interpret(member_info);
//     });
//     assert!(result.is_err());
//   }
// }
