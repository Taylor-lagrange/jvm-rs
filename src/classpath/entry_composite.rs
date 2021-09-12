use super::entry::*;
use std::io::ErrorKind;

pub struct CompositeEntry {
    entrys: Vec<Box<dyn Entry>>,
}

impl CompositeEntry {
    pub fn new_composite_entry(mut path_list: String) -> CompositeEntry {
        let mut entry = CompositeEntry { entrys: Vec::new() };
        if !path_list.ends_with(PATH_LIST_SEPARATOR) {
            path_list += PATH_LIST_SEPARATOR;
        }
        for ent in path_list.split_inclusive(PATH_LIST_SEPARATOR) {
            if let Some(p) = ent.strip_suffix(PATH_LIST_SEPARATOR) {
                if p != "" {
                    entry.entrys.push(new_entry(p.to_string()));
                }
            }
        }
        entry
    }
}

impl super::entry::Entry for CompositeEntry {
    fn read_class(&mut self, class_name: String) -> Result<Vec<u8>, std::io::Error> {
        for ent in self.entrys.iter_mut() {
            let out = (**ent).read_class(class_name.to_owned());
            if let Ok(file) = out {
                return Ok(file);
            }
        }
        Err(std::io::Error::new(
            ErrorKind::NotFound,
            String::from("class not found:") + &class_name,
        ))
    }
    fn string(&self) -> String {
        let mut name = String::new();
        for ent in self.entrys.iter() {
            name += &(**ent).string();
        }
        name
    }
}
