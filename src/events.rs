use std::fs::File;

use crate::{headers::VarHeader, read_bytes_file};

#[derive(Debug, PartialEq)]
pub enum EventValue {
    Char(char),
    Bool(bool),
    Int(i32),
    BitField(u32),
    Float32(f32),
    Float64(f64),
}

impl EventValue {
    pub fn char(&self) -> char {
        if let EventValue::Char(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn bool(&self) -> bool {
        if let EventValue::Bool(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn float_32(&self) -> f32 {
        if let EventValue::Float32(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn float_64(&self) -> f64 {
        if let EventValue::Float64(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn int(&self) -> i32 {
        if let EventValue::Int(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn bitfield(&self) -> u32 {
        if let EventValue::BitField(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
pub struct Event {
    data: Vec<u8>,
}

pub struct Events<'a> {
    // pub file: &'a File,
    pub file: &'a mut File,
    pub length: i32,
    pub buf_offset: i32,
    pub current: i32,
}

impl Iterator for Events<'_> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let from = self.buf_offset + (self.current * self.length);
        match read_bytes_file(&mut self.file, from as usize, self.length as usize) {
            Ok(data) => {
                self.current += 1;
                Some(Event { data })
            }
            Err(_) => None,
        }
    }
}

impl Event {
    pub fn get_by_header(&self, var: &VarHeader) -> Option<EventValue> {
        let offset = var.offset as usize;
        match var.r#type {
            0 => {
                let size = 1;
                Some(EventValue::Char(self.data[offset + size] as char))
            }
            1 => {
                let size = 1;
                Some(EventValue::Bool(self.data[offset + size] != 0))
            }
            2 => {
                let size = 4;
                let value =
                    i32::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(EventValue::Int(value))
            }
            3 => {
                let size = 4;
                let value =
                    u32::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(EventValue::BitField(value))
            }
            4 => {
                let size = 4;
                let value =
                    f32::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(EventValue::Float32(value))
            }
            5 => {
                let size = 8;
                let value =
                    f64::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(EventValue::Float64(value))
            }
            _ => unimplemented!(),
        }
    }
}
