use log4rs;
use structopt::StructOpt;

mod classpath;
mod classfile;

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
    use classpath::entry::Entry;
    let mut cp = Classpath::prase(opt.xjre_option, opt.cp_option);
    if let Ok(file) = cp.read_class(get_class_name(opt.class)) {
        println!("{:?}", file)
    }
}

fn get_class_name(s: String) -> String {
    let p = s.replace(".", "/");
    if p.ends_with("/class") {
        p.strip_suffix("/class").unwrap().to_string() + ".class"
    } else {
        p + ".class"
    }
}
// run example: cargo run -- -x /mnt/d/Development/Java/jdk1.8.0_261/jre/ --class java.lang.Object.class
