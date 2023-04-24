pub mod constants;
mod headers;
mod samples;
mod session_info;

use std::io::{Read, Seek, SeekFrom};

use headers::{DiskHeader, Header, VarHeader, DISK_HEADER_BYTES_SIZE, HEADER_BYTES_SIZE};
use samples::Samples;
use session_info::SessionInfo;
use yore::code_pages::CP1252;

use crate::headers::VAR_HEADER_BYTES_SIZE;

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

pub struct IbtReader {
    file: Box<dyn ReadSeek>,
    pub header: Header,
    pub disk_header: DiskHeader,
    pub vars: Vec<VarHeader>,
    pub session_info: SessionInfo,
}

impl IbtReader {
    pub fn new(mut buffer: Box<dyn ReadSeek>) -> IbtReader {
        let header = Header::from(read_bytes_file(&mut buffer, 0, HEADER_BYTES_SIZE).unwrap());
        let disk_header = DiskHeader::from(
            read_bytes_file(&mut buffer, DISK_HEADER_BYTES_SIZE, HEADER_BYTES_SIZE).unwrap(),
        );
        let session_info_data = read_bytes_file(
            &mut buffer,
            header.sesion_info_offset as usize,
            header.sesion_info_length as usize,
        )
        .unwrap();
        let session_info = serde_yaml::from_str(&CP1252.decode(&session_info_data)).unwrap();

        let vars_data = get_var_header(&mut buffer, &header);
        let vars: Vec<VarHeader> = (0..header.num_vars)
            .map(|n| {
                let start = n as usize * VAR_HEADER_BYTES_SIZE;
                let end = start + VAR_HEADER_BYTES_SIZE;
                VarHeader::from(vars_data[start..end].to_vec())
            })
            .collect();

        IbtReader {
            file: Box::new(buffer),
            header,
            vars,
            disk_header,
            session_info,
        }
    }

    pub fn samples(&mut self) -> Samples {
        Samples {
            current: 0,
            buf_offset: self.header.buf_offset,
            length: self.header.buf_len,
            file: &mut self.file,
        }
    }

    pub fn find_var(&self, name: String) -> Option<VarHeader> {
        self.vars.iter().find(|var| var.name == name).cloned()
    }
}

fn get_var_header(file: &mut dyn ReadSeek, header: &Header) -> Vec<u8> {
    let buffer_size = header.num_vars as usize * VAR_HEADER_BYTES_SIZE;
    read_bytes_file(file, header.var_header_offset as usize, buffer_size).unwrap()
}

fn read_bytes_file(file: &mut dyn ReadSeek, from: usize, size: usize) -> Result<Vec<u8>, ()> {
    let mut buffer: Vec<u8> = vec![0; size];
    file.seek(SeekFrom::Start(from as u64)).unwrap();
    match file.read_exact(&mut buffer).map_err(|_e| ()) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::Flags;
    use crate::samples::{Sample, SampleValue};
    use std::fs::File;

    use super::*;

    #[test]
    fn test_parsing_file() {
        let file = File::open("./test/fixtures/amg.ibt").unwrap();
        let mut reader = IbtReader::new(Box::new(file));
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

        assert_eq!(reader.disk_header.start_date, 1e-45);
        assert_eq!(reader.disk_header.start_time, 0_f64);
        assert_eq!(reader.disk_header.end_time, 1.105135407938237e-309);
        assert_eq!(reader.disk_header.record_count, 0);
        assert_eq!(reader.disk_header.lap_count, 0);

        let weekend_info = &reader.session_info.weekend_info;
        assert_eq!(weekend_info.track_name, "spielberg gp");
        assert_eq!(weekend_info.weekend_options.qualify_scoring, "best lap");

        let camera_info = &reader.session_info.camera_info;
        assert_eq!(camera_info.groups.len(), 22);
        assert_eq!(camera_info.groups[0].group_num, 1);
        assert_eq!(camera_info.groups[0].group_name, "Nose");
        let cameras = &camera_info.groups[0].cameras;
        assert_eq!(cameras.len(), 1);
        assert_eq!(cameras[0].camera_num, 1);
        assert_eq!(cameras[0].camera_name, "CamNose");

        let radio_info = &reader.session_info.radio_info;
        assert_eq!(radio_info.selected_radio_num, 0);
        assert_eq!(radio_info.radios.len(), 1);
        let radio_info = &radio_info.radios[0];
        assert_eq!(radio_info.scanning_is_on, 1);
        assert_eq!(radio_info.frequencies.len(), 7);
        assert_eq!(radio_info.frequencies[0].car_idx, -1);

        let session_info = &reader.session_info.session_info;
        assert_eq!(session_info.sessions.len(), 1);
        let first_session = &session_info.sessions[0];
        assert_eq!(first_session.session_type, "Offline Testing");
        assert_eq!(first_session.results_fastest_lap.len(), 1);
        assert_eq!(first_session.results_fastest_lap[0].car_idx, 255);

        let driver_info = &reader.session_info.driver_info;
        assert_eq!(driver_info.driver_car_idx, 0);
        assert_eq!(driver_info.driver_head_pos_x, -0.643);
        assert_eq!(driver_info.drivers.len(), 1);
        let driver = &driver_info.drivers[0];
        assert_eq!(driver.user_name, "Georgi Martsenkov");
        assert_eq!(driver.car_sponsor_1, 0);

        let split_time_info = &reader.session_info.split_time_info;
        assert_eq!(split_time_info.sectors.len(), 3);
        assert_eq!(split_time_info.sectors[0].sector_num, 0);
        assert_eq!(split_time_info.sectors[1].sector_num, 1);
        assert_eq!(split_time_info.sectors[1].sector_start_pct, 0.271918);

        let car_setup = &reader.session_info.car_setup;
        assert_eq!(
            car_setup.tires.left_rear.tread_remaining,
            "100%, 100%, 100%"
        );
        assert_eq!(car_setup.chassis.rear.arb_setting, 0);
        assert_eq!(car_setup.chassis.in_car_dials.abs_setting, "3 (ABS)");

        let vars = &reader.vars;
        let first = &vars[0];
        assert_eq!(first.r#type, 5);
        assert_eq!(first.offset, 0);
        assert_eq!(first.count, 1);
        assert_eq!(first.count_as_time, 0);
        assert_eq!(first.name, "SessionTime");
        assert_eq!(first.description, "Seconds since session start");
        assert_eq!(first.unit, "s");
        let second = reader.find_var("SessionTick".to_string()).unwrap();
        assert_eq!(second.name, "SessionTick");
        assert_eq!(second.offset, 8);
        let rpm = reader.find_var("RPM".to_string()).unwrap();
        assert_eq!(rpm.name, "RPM");
        let fps = reader.find_var("FrameRate".to_string()).unwrap();
        assert_eq!(fps.name, "FrameRate");
        let flags = reader.find_var("SessionFlags".to_string()).unwrap();
        assert_eq!(flags.name, "SessionFlags");

        let samples: Vec<Sample> = reader.samples().collect();
        assert_eq!(samples.len(), 3371);
        let first_sample = samples[1001].get_by_header(&rpm).unwrap();
        assert_eq!(first_sample, SampleValue::Float32(991.8974));
        let second_sample = samples[1001].get_by_header(&flags).unwrap();
        assert_eq!(second_sample, SampleValue::BitField(268698112));
        assert_eq!(second_sample.bitfield() & Flags::Checkered as u32, 0);
    }

    #[test]
    fn test_parsing_ai_race() {
        let file = File::open("./test/fixtures/ai_race/practice.ibt").unwrap();
        let mut qualify = IbtReader::new(Box::new(file));
        assert_eq!(qualify.header.version, 2);
        assert_eq!(qualify.header.tick_rate, 60);

        assert_eq!(qualify.session_info.weekend_info.session_id, 0);
        assert_eq!(qualify.session_info.session_info.sessions.len(), 2);
        let samples: Vec<Sample> = qualify.samples().collect();
        assert_eq!(samples.len(), 12756);

        let file = File::open("./test/fixtures/ai_race/race.ibt").unwrap();
        let mut race = IbtReader::new(Box::new(file));
        assert_eq!(race.header.version, 2);
        assert_eq!(race.header.tick_rate, 60);

        assert_eq!(race.session_info.session_info.sessions.len(), 2);
        assert_eq!(
            race.session_info.session_info.sessions[0].session_name,
            "QUALIFY"
        );
        assert_eq!(
            race.session_info.session_info.sessions[1].session_name,
            "RACE"
        );
        let drivers = &race.session_info.driver_info.drivers;
        assert_eq!(drivers.len(), 21);
        assert_eq!(drivers[0].user_name, "Georgi Martsenkov");
        assert_eq!(drivers[0].user_id, 290307);
        assert_eq!(drivers[1].user_name, "Kevin Bobbitt");

        let samples: Vec<Sample> = race.samples().collect();
        assert_eq!(samples.len(), 16263);
    }

    #[test]
    fn test_parsing_race() {
        let file = File::open("./test/fixtures/race/practice.ibt").unwrap();

        let mut practice = IbtReader::new(Box::new(file));
        assert_eq!(practice.header.version, 2);
        assert_eq!(practice.header.tick_rate, 60);
        let drivers = &practice.session_info.driver_info.drivers;
        assert_eq!(drivers.len(), 29);
        assert_eq!(drivers[0].user_name, "Pace Car");
        assert_eq!(drivers[0].user_id, -1);
        assert_eq!(drivers[4].user_name, "Georgi Martsenkov");
        assert_eq!(drivers[4].user_id, 290307);

        let samples: Vec<Sample> = practice.samples().collect();
        assert_eq!(samples.len(), 4589);
    }
}
