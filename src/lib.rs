mod events;
mod headers;
mod session_info;

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use events::Events;
use headers::{DiskHeader, Header, VarHeader, DISK_HEADER_BYTES_SIZE, HEADER_BYTES_SIZE};
use session_info::SessionInfo;

use crate::headers::VAR_HEADER_BYTES_SIZE;

pub struct IbtReader {
    file: File,
    pub header: Header,
    pub disk_header: DiskHeader,
    pub vars: Vec<VarHeader>,
    pub session_info: SessionInfo,
}

impl IbtReader {
    pub fn new(file_path: &str) -> IbtReader {
        let mut file = File::open(file_path).expect("Could not load file");
        let header = Header::from(read_bytes_file(&mut file, 0, HEADER_BYTES_SIZE).unwrap());
        let disk_header = DiskHeader::from(
            read_bytes_file(&mut file, DISK_HEADER_BYTES_SIZE, HEADER_BYTES_SIZE).unwrap(),
        );
        let session_info_data = read_bytes_file(
            &mut file,
            header.sesion_info_offset as usize,
            header.sesion_info_length as usize,
        )
        .unwrap();
        let session_info =
            serde_yaml::from_str(std::str::from_utf8(&session_info_data).unwrap()).unwrap();

        let vars_data = get_var_header(&mut file, &header);
        let vars: Vec<VarHeader> = (0..header.num_vars)
            .map(|n| {
                let start = n as usize * VAR_HEADER_BYTES_SIZE;
                let end = start + VAR_HEADER_BYTES_SIZE;
                VarHeader::from(vars_data[start..end].to_vec())
            })
            .collect();

        IbtReader {
            file,
            header,
            vars,
            disk_header,
            session_info,
        }
    }

    pub fn events(&mut self) -> Events {
        Events {
            current: 0,
            buf_offset: self.header.buf_offset,
            length: self.header.buf_len,
            file: &mut self.file,
        }
    }
}

fn get_var_header(file: &mut File, header: &Header) -> Vec<u8> {
    //    const numberOfVariables = telemetryHeader.numVars
    //   const startPosition = telemetryHeader.varHeaderOffset
    //    const fullBufferSize = numberOfVariables * VAR_HEADER_SIZE_IN_BYTES
    let buffer_size = header.num_vars as usize * VAR_HEADER_BYTES_SIZE;
    read_bytes_file(file, header.var_header_offset as usize, buffer_size).unwrap()
}

fn read_bytes_file(file: &mut File, from: usize, size: usize) -> Result<Vec<u8>, ()> {
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    buffer.resize(size, 0);
    file.seek(SeekFrom::Start(from as u64)).unwrap();
    match file.read_exact(&mut buffer).map_err(|_e| ()) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::events::{Event, EventValue};

    use super::*;

    #[test]
    fn test_new() {
        let mut reader = IbtReader::new("./test/fixtures/amg.ibt");
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

        let weekend_info = &reader.session_info.weekend_info;
        assert_eq!(weekend_info.track_name, "spielberg gp");
        assert_eq!(weekend_info.weekend_options.qualify_scoring, "best lap");

        let vars = &reader.vars;
        let first = &vars[0];
        assert_eq!(first.r#type, 5);
        assert_eq!(first.offset, 0);
        assert_eq!(first.count, 1);
        assert_eq!(first.count_as_time, 0);
        assert_eq!(first.name, "SessionTime");
        assert_eq!(first.description, "Seconds since session start");
        assert_eq!(first.unit, "s");
        let second = &vars[1];
        assert_eq!(second.name, "SessionTick");
        assert_eq!(second.offset, 8);
        let rpm = &vars[55].clone();
        assert_eq!(rpm.name, "RPM");
        let fps = &vars[20].clone();
        assert_eq!(fps.name, "FrameRate");

        let events: Vec<Event> = reader.events().collect();
        assert_eq!(events.len(), 3371);
        let first_event = events[1001].get_by_header(rpm).unwrap();
        assert_eq!(first_event, EventValue::Float32(991.8974));
    }
}
