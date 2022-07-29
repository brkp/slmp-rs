#[derive(Debug, Clone, Copy)]
pub struct Header {
    serial_no: Option<u16>,
    dst_ne_no: u8,
    dst_st_no: u8,
    dst_md_no: u16,
    dst_mt_no: u8,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            serial_no: None,
            dst_ne_no: 0x00,
            dst_st_no: 0xff,
            dst_md_no: 0x03ff,
            dst_mt_no: 0x00,
        }
    }
}

impl Header {
    fn build(&self, buf: &mut Vec<u8>) {
        todo!()
    }

    fn from(&self, data: &[u8]) -> Self {
        todo!()
    }
}
