use log::debug;
use std::io::ErrorKind;
use zip::ZipArchive;

pub struct ZipEntry {
  abs_dir: std::path::PathBuf,
  archive: zip::ZipArchive<std::fs::File>,
}

impl ZipEntry {
  pub fn new_zip_entry(path: String) -> ZipEntry {
    let mut p = std::path::PathBuf::new();
    p.push(&path);
    let archive = ZipArchive::new(
      std::fs::File::open(p.as_path()).expect(&format!("Couldn't open file {}", &path)),
    )
    .expect(&format!("Couldn't open file {}", &path));
    debug!("{} loads complete", path);
    let mut entry = ZipEntry {
      abs_dir: p,
      archive: archive,
    };
    entry.abs_dir.push(path);
    entry
  }
}

impl super::entry::Entry for ZipEntry {
  fn read_class(&mut self, class_name: String) -> Result<Vec<u8>, std::io::Error> {
    let real_name = self.get_class_name(class_name);
    let file = self.archive.by_name(real_name.as_str());
    match file {
      Ok(mut con) => {
        use std::io::Read;
        let mut content = Vec::new();
        con.read_to_end(&mut content)?;
        Ok(content)
      }
      _ => Err(std::io::Error::new(
        ErrorKind::NotFound,
        String::from("class not found:") + &real_name,
      )),
    }
  }
  fn string(&self) -> String {
    self.abs_dir.to_str().unwrap().to_string()
  }
}
