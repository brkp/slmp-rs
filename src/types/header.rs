use crate::Result;

/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub serial_no: Option<u16>,
    pub dst_ne_no: u8,
    pub dst_st_no: u8,
    pub dst_md_no: u16,
    pub dst_mt_no: u8,
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
    pub fn build(&self, buf: &mut Vec<u8>) {
        match self.serial_no {
            None => buf.extend_from_slice(&[0x50, 00]),
            Some(u) => {
                buf.extend_from_slice(&[0x54, 0x00]);
                buf.extend_from_slice(&u.to_le_bytes());
                buf.extend_from_slice(&[0x00, 0x00]);
            }
        }
        buf.extend_from_slice(&[self.dst_ne_no, self.dst_st_no]);
        buf.extend_from_slice(&self.dst_md_no.to_le_bytes());
        buf.push(self.dst_mt_no);
    }

    pub fn from(buf: &[u8]) -> Result<Self> {
        match buf.get(0) {
            Some(n) if *n == 0xd0 && buf.len() != 0x7 => return Err("invalid buffer".into()),
            Some(n) if *n == 0xd4 && buf.len() != 0xb => return Err("invalid buffer".into()),
            None                                      => return Err("invalid buffer".into()),
            _ => (),
        }

        let mut header = Self::default();
        let mut index;

        match buf[0] {
            0xd0 => { index = 2; header.serial_no = None; },
            0xd4 => { index = 6; header.serial_no = Some(u16::from_le_bytes([buf[2], buf[3]])); }
            _    => unreachable!()
        }

        header.dst_ne_no = buf[index]; index += 1;
        header.dst_st_no = buf[index]; index += 1;
        header.dst_md_no = u16::from_le_bytes([buf[index], buf[index + 1]]); index += 2;
        header.dst_mt_no = buf[index];

        Ok(header)
    }
}
