use super::entry_composite::*;
use super::entry_dir::*;
use super::entry_wildcard::*;
use super::entry_zip::*;

pub const PATH_LIST_SEPARATOR: &str = ";";

pub trait Entry {
  fn read_class(&mut self, class_name: String) -> Result<Vec<u8>, std::io::Error>;
  fn string(&self) -> String;
  fn get_class_name(&self, s: String) -> String {
    let p = s.replace(".", "/");
    if p.ends_with("/class") {
      p.strip_suffix("/class").unwrap().to_string() + ".class"
    } else {
      p + ".class"
    }
  }
}

pub fn new_entry(path: String) -> Box<dyn Entry> {
  if path.contains(PATH_LIST_SEPARATOR) {
    return Box::new(CompositeEntry::new_composite_entry(path));
  }
  if path.ends_with("*") {
    return Box::new(new_wildcard_entry(path));
  }
  if path.ends_with(".jar")
    || path.ends_with(".JAR")
    || path.ends_with(".zip")
    || path.ends_with(".ZIP")
  {
    return Box::new(ZipEntry::new_zip_entry(path));
  }
  return Box::new(DirEntry::new_dir_entry(path));
}
