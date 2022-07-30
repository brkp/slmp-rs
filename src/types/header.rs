use crate::Result;

/// Shared data between a `Request` and a `Response`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// Encodes the `Subheader` part of a `Request`/`Response`.
    /// The `Subheader` differs depending on whether or not a serial no. is added.
    pub serial_no: Option<u16>,
    /// Request destination network No.
    pub dst_ne_no: u8,
    /// Request destination station No.
    pub dst_st_no: u8,
    /// Request destination module I/O No.
    pub dst_md_no: u16,
    /// Request destination multidrop station No.
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
    /// Write each `Header` field into some `&mut Vec<u8>` buffer in little endian.
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

    /// Build a `Header` from some `&[u8]` buffer.
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
            // if the header starts with `0xd0`, there's no serial no. to parse
            0xd0 => { index = 2; header.serial_no = None; },
            // if the header starts with `0xd4`, parse the serial no.
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

#[cfg(test)]
mod tests {
    use super::Header;

    #[test]
    fn test_header_build() {
        let mut buf1 = Vec::new();
        let mut buf2 = Vec::new();

        let hdr1 = Header::default();
        let hdr2 = Header {
            serial_no: Some(0x1234),
            dst_ne_no: 0xde,
            dst_st_no: 0xad,
            dst_md_no: 0xefbe,
            dst_mt_no: 0xee,
        };

        hdr1.build(&mut buf1);
        hdr2.build(&mut buf2);

        assert_eq!(&buf1, &[0x50, 0x00, 0x00, 0xff, 0xff, 0x03, 0x00]);
        assert_eq!(&buf2, &[0x54, 0x00, 0x34, 0x12, 0x00, 0x00, 0xde, 0xad, 0xbe, 0xef, 0xee]);
    }

    #[test]
    fn test_header_from() {
        let buf1 = [0xd0, 0x00, 0x00, 0xff, 0xff, 0x03, 0x00];
        let buf2 = [0xd4, 0x00, 0x34, 0x12, 0x00, 0x00, 0xde, 0xad, 0xbe, 0xef, 0xee];

        assert!(Header::from(&buf1).is_ok());
        assert!(Header::from(&buf2).is_ok());

        assert!(Header::from(&[]).is_err());
        assert!(Header::from(&[0xd0, 0x00]).is_err());
    }
}
