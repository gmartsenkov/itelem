mod headers;
mod session_info;

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    usize,
};

use headers::{DiskHeader, Header, DISK_HEADER_BYTES_SIZE, HEADER_BYTES_SIZE};
use session_info::SessionInfo;

pub struct IbtReader {
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
