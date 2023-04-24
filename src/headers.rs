use yore::code_pages::CP1252;

pub const HEADER_BYTES_SIZE: usize = 112;
pub const DISK_HEADER_BYTES_SIZE: usize = 32;
pub const VAR_HEADER_BYTES_SIZE: usize = 144;

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

#[derive(Debug, Clone)]
pub struct VarHeader {
    pub r#type: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_time: i8,
    pub name: String,
    pub description: String,
    pub unit: String,
}

impl From<Vec<u8>> for VarHeader {
    fn from(data: Vec<u8>) -> VarHeader {
        VarHeader {
            r#type: i32::from_le_bytes(data[0..4].try_into().unwrap()),
            offset: i32::from_le_bytes(data[4..8].try_into().unwrap()),
            count: i32::from_le_bytes(data[8..12].try_into().unwrap()),
            count_as_time: i8::from_le_bytes(data[12..13].try_into().unwrap()),
            // padding here, 16 byte align (3 bytes)
            name: CP1252.decode(&data[16..48]).to_string().replace('\0', ""),
            description: CP1252.decode(&data[48..112]).to_string().replace('\0', ""),
            unit: CP1252.decode(&data[112..144]).to_string().replace('\0', ""),
        }
    }
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
