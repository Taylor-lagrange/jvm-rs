pub struct DirEntry {
  abs_dir: std::path::PathBuf,
}

impl DirEntry {
  pub fn new_dir_entry(path: String) -> DirEntry {
    let mut entry= DirEntry {
      abs_dir: std::path::PathBuf::new(),
    };
    entry.abs_dir.push(path);
    entry
  }
}

impl super::entry::Entry for DirEntry {
  fn read_class(&mut self, class_name: String) -> Result<Vec<u8>, std::io::Error> {
    self.abs_dir.push(class_name);
    let file = std::fs::File::open(self.abs_dir.as_os_str());
    self.abs_dir.pop();
    match file {
      Ok(mut file) => {
        use std::io::Read;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
      }
      Err(err) => Err(err),
    }
  }
  fn string(&self) -> String {
    self.abs_dir.to_str().unwrap().to_string()
  }
}
