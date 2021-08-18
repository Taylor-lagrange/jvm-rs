use crate::classfile::attribute_info::*;
use crate::instructions::*;
use log4rs;
use structopt::StructOpt;

mod classfile;
mod classpath;
mod instructions;
mod runtime;
mod utils;

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
    use classfile::class_file::*;
    use classpath::classpath::*;
    use classpath::entry::Entry;
    let mut cp = Classpath::prase(opt.xjre_option, opt.cp_option);
    if let Ok(file) = cp.read_class(opt.class) {
        // println!("{:?}", file);
        if let Ok(class_file) = ClassFile::parse(file) {
            // let method = class_file.get_main_method();
            // instructions::interpreter::interpret(method.clone());
            // if let AttributeInfo::Code {
            //     max_stack,
            //     max_locals,
            //     code,
            //     ..
            // } = method.code_attribute()
            // {
            //     println!("ms:{} ml:{} c:{:?}", max_stack, max_locals, code)
            // }
            class_file.print_class_info()
        }
    }
}

// run example: cargo run -- -x /mnt/d/Development/Java/jdk1.8.0_261/jre/ --class java.lang.Object.class
