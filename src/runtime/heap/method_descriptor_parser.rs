#[derive(Default)]
pub struct MethodDescriptor {
  pub parameter_type: Vec<String>,
  pub return_type: String,
}

#[derive(Default)]
pub struct MethodDescriptorParser {
  descriptor: String,
  raw: Vec<u8>,
  offset: usize,
  parsed: MethodDescriptor,
}

impl MethodDescriptorParser {
  pub fn parse(descriptor: String) -> MethodDescriptor {
    let mut parse: MethodDescriptorParser = Default::default();
    parse.raw = descriptor.as_bytes().to_vec();
    parse.descriptor = descriptor;
    parse.process();
    parse.parsed
  }
  fn process(&mut self) {
    self.start_params();
    self.parse_param_types();
    self.end_params();
    self.parse_return_type();
    self.finish();
  }
  fn start_params(&mut self) {
    if self.read_u8() as char != '(' {
      self.panic();
    }
  }
  fn end_params(&mut self) {
    if self.read_u8() as char != ')' {
      self.panic();
    }
  }
  fn finish(&mut self) {
    if self.offset != self.raw.len() {
      self.panic();
    }
  }
  fn panic(&self) {
    panic!("BAD descriptor: {}", self.descriptor);
  }
  fn read_u8(&mut self) -> u8 {
    self.offset += 1;
    self.raw[self.offset - 1]
  }
  fn unread_u8(&mut self) {
    self.offset -= 1;
  }
  fn parse_param_types(&mut self) {
    loop {
      let t = self.parse_field_type();
      if t != "" {
        self.parsed.parameter_type.push(t)
      } else {
        break;
      }
    }
  }
  fn parse_return_type(&mut self) {
    if self.read_u8() as char == 'V' {
      self.parsed.return_type = "V".to_string();
      return;
    }
    self.unread_u8();
    let t = self.parse_field_type();
    if t != "" {
      self.parsed.return_type = t;
      return;
    }
    self.panic();
  }
  fn parse_field_type(&mut self) -> String {
    match self.read_u8() as char {
      'B' => "B".to_string(),
      'C' => "C".to_string(),
      'D' => "D".to_string(),
      'F' => "F".to_string(),
      'I' => "I".to_string(),
      'J' => "J".to_string(),
      'S' => "S".to_string(),
      'Z' => "Z".to_string(),
      'L' => self.parse_object_type(),
      '[' => self.parse_array_type(),
      _ => {
        self.unread_u8();
        "".to_string()
      }
    }
  }
  fn parse_object_type(&mut self) -> String {
    let unread = self.raw.get(self.offset..).unwrap();
    let semicolon_index = unread.iter().position(|&r| r as char == ';');
    match semicolon_index {
      None => {
        self.panic();
        String::new()
      }
      Some(index) => {
        let obj_start = self.offset - 1;
        let obj_end = self.offset + index + 1;
        self.offset = obj_end;
        String::from(std::str::from_utf8(self.raw.get(obj_start..obj_end).unwrap()).unwrap())
      }
    }
  }
  fn parse_array_type(&mut self) -> String {
    let arr_start = self.offset - 1;
    self.parse_field_type();
    let arr_end = self.offset;
    String::from(std::str::from_utf8(self.raw.get(arr_start..arr_end).unwrap()).unwrap())
  }
}
