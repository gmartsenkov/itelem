use std::{fs::File, io::Read};

pub struct Header {
    pub version: u32,
    pub tick_rate: u32
}

impl From<Vec<u8>> for Header {
    fn from(data : Vec<u8>) -> Header {
        Header{
            version: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            tick_rate: u32::from_le_bytes(data[8..12].try_into().unwrap()),
        }
    }

}
pub struct IbtReader {
    file: File,
    pub header: Header
}

impl IbtReader {
    pub fn new(file_path : &str) -> IbtReader {
        let mut file = File::open(file_path).expect("Could not load file");
        let head = Self::header(&mut file);
        IbtReader { file, header: Header::from(head.to_vec()) }
    }

    pub fn header(file : &mut File) -> [u8; 112] {
        let mut buffer: [u8; 112] = [0; 112];
        file.read_exact(&mut buffer).unwrap();
        println!("Header {:?}", &buffer);
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        println!("Bob");
        let reader = IbtReader::new("./test/fixtures/amg.ibt");
        assert_eq!(reader.header.version, 2);
        assert_eq!(reader.header.tick_rate, 60);
    }

}
