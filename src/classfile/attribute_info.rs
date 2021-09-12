use super::class_reader::*;
use super::constant_pool::*;

#[derive(Clone, Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Clone, Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

#[derive(Clone, Debug)]
pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Clone, Debug)]
pub enum AttributeInfo {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>,
    },
    ConstantValue {
        constant_value_index: usize,
    },
    Deprecated,
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumberTableEntry>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariableTableEntry>,
    },
    SourceFile {
        source_file_index: usize,
    },
    Synthetic,
    Unparsed {
        name_index: u16,
        length: u32,
        info: Vec<u8>,
    },
    Nil,
}

impl AttributeInfo {
    pub fn read_attributes(reader: &mut ClassReader, cp: &ConstantPool) -> Vec<AttributeInfo> {
        let attribute_count = reader.read_u16();
        let mut v = Vec::new();
        for _ in 0..attribute_count {
            v.push(AttributeInfo::read_attribute(reader, cp))
        }
        v
    }
    fn read_attribute(reader: &mut ClassReader, cp: &ConstantPool) -> AttributeInfo {
        let attr_name_index = reader.read_u16();
        let attr_len = reader.read_u32();
        match cp.get_utf8(attr_name_index as usize).as_str() {
            "Code" => {
                let max_stack = reader.read_u16();
                let max_locals = reader.read_u16();
                let code_len = reader.read_u32();
                let code = reader.read_bytes(&(code_len as usize));
                let exception_table = AttributeInfo::read_exception_table(reader);
                let attributes = AttributeInfo::read_attributes(reader, cp);
                AttributeInfo::Code {
                    max_stack: max_stack,
                    max_locals: max_locals,
                    code: code,
                    exception_table: exception_table,
                    attributes: attributes,
                }
            }
            "ConstantValue" => AttributeInfo::ConstantValue {
                constant_value_index: reader.read_u16() as usize,
            },
            "Deprecated" => AttributeInfo::Deprecated,
            "Exceptions" => AttributeInfo::Exceptions {
                exception_index_table: reader.read_u16s(),
            },
            "LineNumberTable" => {
                let line_number_table_length = reader.read_u16();
                let mut table = Vec::new();
                for _ in 0..line_number_table_length {
                    table.push(LineNumberTableEntry {
                        start_pc: reader.read_u16(),
                        line_number: reader.read_u16(),
                    })
                }
                AttributeInfo::LineNumberTable {
                    line_number_table: table,
                }
            }
            "LocalVariableTable" => {
                let local_variable_table_len = reader.read_u16();
                let mut table = Vec::new();
                for _ in 0..local_variable_table_len {
                    table.push(LocalVariableTableEntry {
                        start_pc: reader.read_u16(),
                        length: reader.read_u16(),
                        name_index: reader.read_u16(),
                        descriptor_index: reader.read_u16(),
                        index: reader.read_u16(),
                    })
                }
                AttributeInfo::LocalVariableTable {
                    local_variable_table: table,
                }
            }
            "SourceFile" => AttributeInfo::SourceFile {
                source_file_index: reader.read_u16() as usize,
            },
            "Synthetic" => AttributeInfo::Synthetic,
            _ => AttributeInfo::Unparsed {
                name_index: attr_name_index,
                length: attr_len,
                info: reader.read_bytes(&(attr_len as usize)),
            },
        }
    }
    fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
        let exception_table_len = reader.read_u16();
        let mut table = Vec::new();
        for _ in 0..exception_table_len {
            table.push(ExceptionTableEntry {
                start_pc: reader.read_u16(),
                end_pc: reader.read_u16(),
                handler_pc: reader.read_u16(),
                catch_type: reader.read_u16(),
            })
        }
        table
    }
}
