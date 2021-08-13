use super::entry::{new_entry, Entry};
use std::env;

pub struct Classpath {
  boot_classpath: Box<dyn Entry>,
  ext_classpath: Box<dyn Entry>,
  user_classpath: Box<dyn Entry>,
}

// -Xjre选项          解析启动类路径和扩展类路径
// -classpath/-cp选项 解析用户类路径

impl Classpath {
  pub fn prase(jre_option: Option<String>, cp_option: Option<String>) -> Classpath {
    let jre_dir = std::path::PathBuf::from(Classpath::get_jre_dir(jre_option));
    let mut jre_lib_path = jre_dir.clone();
    let mut jre_ext_path = jre_dir.clone();
    jre_lib_path.push("lib");
    jre_lib_path.push("*");
    jre_ext_path.push("lib");
    jre_ext_path.push("ext");
    jre_ext_path.push("*");
    let user_path = match cp_option {
      Some(p) => p,
      _ => String::from("."),
    };
    Classpath {
      boot_classpath: new_entry(jre_lib_path.into_os_string().into_string().unwrap()),
      ext_classpath: new_entry(jre_ext_path.into_os_string().into_string().unwrap()),
      user_classpath: new_entry(user_path),
    }
  }
  fn get_jre_dir(jre_option: Option<String>) -> String {
    if let Some(jre) = jre_option {
      if Classpath::exists(&jre) {
        return jre;
      }
    }
    if Classpath::exists(&String::from("./jre")) {
      return "./jre".to_string();
    }
    if let Ok(p) = env::var("JAVA_HOME") {
      return p;
    }
    panic!("can't find jre dir!")
  }
  fn exists(path: &String) -> bool {
    std::path::PathBuf::from(path).exists()
  }
}

impl super::entry::Entry for Classpath {
  fn read_class(&mut self, class_name: String) -> Result<Vec<u8>, std::io::Error> {
    let mut classfile = self.boot_classpath.read_class(class_name.clone());
    if classfile.is_ok() {
      return classfile;
    }
    classfile = self.ext_classpath.read_class(class_name.clone());
    if classfile.is_ok() {
      return classfile;
    }
    self.user_classpath.read_class(class_name)
  }
  fn string(&self) -> String {
    self.user_classpath.string()
  }
}
