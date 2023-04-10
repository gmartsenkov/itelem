mod session_info;

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    usize,
};

use session_info::SessionInfo;

const HEADER_BYTES_SIZE: usize = 112;
const DISK_HEADER_BYTES_SIZE: usize = 32;

pub struct DiskHeader {
    pub start_date: f32,
    pub start_time: f64,
    pub end_time: f64,
    pub lap_count: i32,
    pub record_count: i32,
}

pub struct Header {
    pub version: i32,
    pub status: i32,
    pub tick_rate: i32,
    pub sesion_info_update: i32,
    pub sesion_info_offset: i32,
    pub sesion_info_length: i32,
    pub num_vars: i32,
    pub var_header_offset: i32,
    pub num_buf: i32,
    pub buf_len: i32,
    pub buf_offset: i32,
}

impl From<Vec<u8>> for DiskHeader {
    fn from(data: Vec<u8>) -> DiskHeader {
        DiskHeader {
            start_date: f32::from_le_bytes(data[0..4].try_into().unwrap()),
            start_time: f64::from_le_bytes(data[8..16].try_into().unwrap()),
            end_time: f64::from_le_bytes(data[16..24].try_into().unwrap()),
            lap_count: i32::from_le_bytes(data[24..28].try_into().unwrap()),
            record_count: i32::from_le_bytes(data[28..32].try_into().unwrap()),
        }
    }
}

impl From<Vec<u8>> for Header {
    fn from(data: Vec<u8>) -> Header {
        Header {
            version: i32::from_le_bytes(data[0..4].try_into().unwrap()),
            status: i32::from_le_bytes(data[4..8].try_into().unwrap()),
            tick_rate: i32::from_le_bytes(data[8..12].try_into().unwrap()),
            sesion_info_update: i32::from_le_bytes(data[12..16].try_into().unwrap()),
            sesion_info_length: i32::from_le_bytes(data[16..20].try_into().unwrap()),
            sesion_info_offset: i32::from_le_bytes(data[20..24].try_into().unwrap()),
            num_vars: i32::from_le_bytes(data[24..28].try_into().unwrap()),
            var_header_offset: i32::from_le_bytes(data[28..32].try_into().unwrap()),
            num_buf: i32::from_le_bytes(data[32..36].try_into().unwrap()),
            buf_len: i32::from_le_bytes(data[36..40].try_into().unwrap()),
            buf_offset: i32::from_le_bytes(data[52..56].try_into().unwrap()),
        }
    }
}
pub struct IbtReader {
    file: File,
    pub header: Header,
    pub disk_header: DiskHeader,
    pub session_info: SessionInfo,
}

impl IbtReader {
    pub fn new(file_path: &str) -> IbtReader {
        let mut file = File::open(file_path).expect("Could not load file");
        let header = Header::from(read_bytes_file(&mut file, 0, HEADER_BYTES_SIZE));
        let disk_header = DiskHeader::from(read_bytes_file(
            &mut file,
            DISK_HEADER_BYTES_SIZE,
            HEADER_BYTES_SIZE,
        ));
        let session_info_data = read_bytes_file(
            &mut file,
            header.sesion_info_offset as usize,
            header.sesion_info_length as usize,
        );
        println!("{}", std::str::from_utf8(&session_info_data).unwrap());
        IbtReader {
            file,
            header,
            disk_header,
            session_info: serde_yaml::from_str(std::str::from_utf8(&session_info_data).unwrap())
                .unwrap(),
        }
    }
}

fn read_bytes_file(file: &mut File, from: usize, size: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    buffer.resize(size, 0);
    file.seek(SeekFrom::Start(from as u64)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reader = IbtReader::new("./test/fixtures/amg.ibt");
        assert_eq!(reader.header.version, 2);
        assert_eq!(reader.header.tick_rate, 60);
        assert_eq!(reader.header.status, 1);
        assert_eq!(reader.header.sesion_info_update, 0);
        assert_eq!(reader.header.sesion_info_offset, 38592);
        assert_eq!(reader.header.sesion_info_length, 13488);
        assert_eq!(reader.header.num_vars, 267);
        assert_eq!(reader.header.var_header_offset, 144);
        assert_eq!(reader.header.num_buf, 1);
        assert_eq!(reader.header.buf_len, 1039);
        assert_eq!(reader.header.buf_offset, 52080);

        assert_eq!(reader.disk_header.start_date, 1.401298464324817e-45);
        assert_eq!(reader.disk_header.start_time, 0_f64);
        assert_eq!(reader.disk_header.end_time, 1.105135407938237e-309);
        assert_eq!(reader.disk_header.record_count, 0);
        assert_eq!(reader.disk_header.lap_count, 0);

        assert_eq!(reader.session_info.weekend_info.track_name, "spielberg gp");
        assert_eq!(
            reader
                .session_info
                .weekend_info
                .weekend_options
                .qualify_scoring,
            "best lap"
        );
    }
}
