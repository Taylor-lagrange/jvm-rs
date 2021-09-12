use log4rs;
use structopt::StructOpt;

mod classfile;
mod classpath;
mod instructions;
mod native;
mod runtime;
mod utils;

#[macro_use]
extern crate lazy_static;

#[derive(StructOpt, Debug)]
#[structopt(name = "Usage")]
struct Opt {
    // short and long flags (-v, --version) will be deduced from the field's name
    #[structopt(short, long)]
    version: bool,
    // short and long flags (-v, --version) will be deduced from the field's name
    #[structopt(short, long)]
    help: bool,
    #[structopt(short = "cp", long)]
    cp_option: Option<String>,
    #[structopt(short, long)]
    xjre_option: Option<String>,
    #[structopt(long)]
    class: String,
    #[structopt(short, long)]
    args: Vec<String>,
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let opt = Opt::from_args();
    if opt.version {
        println!("Java Rust 64-Bit Server VM");
        return;
    }
    if opt.help {
        println!("Usage: %s [-options] --class <class> --args <args...>\n");
        return;
    }
    start_jvm(opt)
}

fn start_jvm(opt: Opt) {
    println!("JVM start! class: {} args: {:?} \n", opt.class, opt.args);
    use classpath::classpath::*;
    use instructions::interpreter::*;
    use runtime::heap::class_loader::*;
    use std::rc::Rc;

    let cp = Classpath::prase(opt.xjre_option, opt.cp_option);
    let class_loader = ClassLoader::new(cp);
    let main_class = ClassLoader::load_class(Rc::downgrade(&class_loader), &opt.class);
    let main_method = main_class
        .upgrade()
        .unwrap()
        .borrow_mut()
        .get_main_method()
        .upgrade();
    if main_method.is_none() {
        print!("Main method not found in class {}\n", opt.class);
    } else {
        interpret(main_method.unwrap(), opt.args);
    }
}

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test
// not when you run cargo build .
// -x ./resources --class ./resources/tests/FibonacciTest.class
// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::fs::File;
//     use std::io::Write;
//     use pprof::protos::Message;
//
//     #[test]
//     fn pprof_test_fibonacci() {
//         let guard = pprof::ProfilerGuard::new(100).unwrap();
//         let fibonacci = Opt {
//             version: false,
//             help: false,
//             cp_option: None,
//             xjre_option: Some("./resources".to_string()),
//             class: "./resources/tests/FibonacciTest.class".to_string(),
//             args: Vec::new(),
//         };
//         println!("jvm start!");
//         start_jvm(fibonacci);
//         if let Ok(report) = guard.report().build() {
//             let file = File::create("flamegraph.svg").unwrap();
//             report.flamegraph(file).unwrap();
//             let mut pb_file = File::create("profile.pb").unwrap();
//             let profile = report.pprof().unwrap();
//             let mut content = Vec::new();
//             profile.encode(&mut content).unwrap();
//             pb_file.write_all(&content).unwrap();
//         };
//     }
// }
