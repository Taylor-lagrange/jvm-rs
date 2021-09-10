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

// run example: cargo run -- -x /mnt/d/Development/Java/jdk1.8.0_261/jre/ --class java.lang.Object.class
