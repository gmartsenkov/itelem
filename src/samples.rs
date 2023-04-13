use crate::{headers::VarHeader, read_bytes_file, ReadSeek};

#[derive(Debug, PartialEq)]
pub enum SampleValue {
    Char(char),
    Bool(bool),
    Int(i32),
    BitField(u32),
    Float32(f32),
    Float64(f64),
}

impl SampleValue {
    pub fn char(&self) -> char {
        if let SampleValue::Char(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn bool(&self) -> bool {
        if let SampleValue::Bool(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn float_32(&self) -> f32 {
        if let SampleValue::Float32(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn float_64(&self) -> f64 {
        if let SampleValue::Float64(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn int(&self) -> i32 {
        if let SampleValue::Int(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }

    pub fn bitfield(&self) -> u32 {
        if let SampleValue::BitField(x) = self {
            x.clone()
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
pub struct Sample {
    data: Vec<u8>,
}

pub struct Samples<'a> {
    pub file: &'a mut dyn ReadSeek,
    pub length: i32,
    pub buf_offset: i32,
    pub current: i32,
}

impl Iterator for Samples<'_> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let from = self.buf_offset + (self.current * self.length);
        match read_bytes_file(&mut self.file, from as usize, self.length as usize) {
            Ok(data) => {
                self.current += 1;
                Some(Sample { data })
            }
            Err(_) => None,
        }
    }
}

impl Sample {
    pub fn get_by_header(&self, var: &VarHeader) -> Option<SampleValue> {
        let offset = var.offset as usize;
        match var.r#type {
            0 => {
                let size = 1;
                Some(SampleValue::Char(self.data[offset + size] as char))
            }
            1 => {
                let size = 1;
                Some(SampleValue::Bool(self.data[offset + size] != 0))
            }
            2 => {
                let size = 4;
                let value =
                    i32::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(SampleValue::Int(value))
            }
            3 => {
                let size = 4;
                let value =
                    u32::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(SampleValue::BitField(value))
            }
            4 => {
                let size = 4;
                let value =
                    f32::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(SampleValue::Float32(value))
            }
            5 => {
                let size = 8;
                let value =
                    f64::from_le_bytes(self.data[offset..(offset + size)].try_into().unwrap());
                Some(SampleValue::Float64(value))
            }
            _ => unimplemented!(),
        }
    }
}
