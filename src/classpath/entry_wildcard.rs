use super::entry::PATH_LIST_SEPARATOR;
use super::entry_composite::*;

pub fn new_wildcard_entry(path_list: String) -> CompositeEntry {
    let mut str = String::new();
    let paths = std::fs::read_dir(path_list.strip_suffix("*").unwrap()).unwrap();
    for path in paths {
        if let Ok(info) = path {
            if info.file_type().unwrap().is_dir() {
                continue;
            }
            let path = String::from(info.path().to_str().unwrap());
            if path.ends_with(".jar") || path.ends_with(".JAR") {
                str += &(path + PATH_LIST_SEPARATOR);
            }
        }
    }
    CompositeEntry::new_composite_entry(str.to_string())
}
