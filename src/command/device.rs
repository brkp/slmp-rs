use crate::command::{command, Cmd};
use crate::{Command, Device, DeviceType, Error, Result};

command!(struct ReadCmd, device: Device, addr: u16, count: u16);
// Write doesn't need to decode anything
command!(struct WriteCmd, device: Device, addr: u16, len: u16, data: Vec<u8>);

impl ReadCmd {
    fn encode(&self, buf: &mut Vec<u8>) -> usize {
        let cmd = Command::DeviceRead as u16;
        let sub = match self.device.get_type() {
            DeviceType::Bit => 0x0001u16,
            DeviceType::Word => 0x0000u16,
            DeviceType::DoubleWord => 0x0000u16,
        };

        buf.extend_from_slice(&cmd.to_le_bytes());
        buf.extend_from_slice(&sub.to_le_bytes());
        buf.extend_from_slice(&self.addr.to_le_bytes());
        buf.push(0x00); // this is here to pad the addr field to `3` bytes
        buf.push(self.device as u8);
        buf.extend_from_slice(&self.count.to_le_bytes());

        10
    }
}

impl WriteCmd {
    fn encode(&self, buf: &mut Vec<u8>) -> usize {
        let cmd = Command::DeviceWrite as u16;
        let sub = match self.device.get_type() {
            DeviceType::Bit => 0x0001u16,
            DeviceType::Word => 0x0000u16,
            DeviceType::DoubleWord => 0x0000u16,
        };

        buf.extend_from_slice(&cmd.to_le_bytes());
        buf.extend_from_slice(&sub.to_le_bytes());
        buf.extend_from_slice(&self.addr.to_le_bytes());
        buf.push(0x00); // this is here to pad the addr field to `3` bytes
        buf.push(self.device as u8);
        buf.extend_from_slice(&self.len.to_le_bytes());
        buf.extend_from_slice(&self.data);

        10 + self.data.len()
    }
}

impl Cmd<Vec<bool>> for ReadCmd {
    fn decode(&self, buf: &[u8]) -> Result<Vec<bool>> {
        let data = buf
            .into_iter()
            .flat_map(|n| [(*n & 0x10) != 0, (*n & 1) != 0])
            .collect::<Vec<bool>>();

        if data.len() < self.count as usize {
            return Err("invalid buffer".into());
        }

        Ok(data[..self.count as usize].to_vec())
    }
}

impl Cmd<Vec<u16>> for ReadCmd {
    fn decode(&self, buf: &[u8]) -> Result<Vec<u16>> {
        // `buf` must have an even length
        if buf.len() % 2 != 0 {
            return Err("invalid buffer".into());
        }

        Ok(buf
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_le_bytes([a[0], a[1]]))
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_encode() {
        let (mut buf1, mut buf2): (Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new());
        let cmd1 = ReadCmd::generate(Device::D, 200, 8); // word device
        let cmd2 = ReadCmd::generate(Device::M, 200, 1); // bit device

        cmd1.encode(&mut buf1);
        cmd2.encode(&mut buf2);

        assert_eq!(
            buf1,
            [0x01, 0x04, 0x00, 0x00, 0xc8, 0x00, 0x00, 0xa8, 0x08, 0x00]
        );
        assert_eq!(
            buf2,
            [0x01, 0x04, 0x01, 0x00, 0xc8, 0x00, 0x00, 0x90, 0x01, 0x00]
        );
    }

    #[test]
    fn test_read_decode() {
        let cmd1 = ReadCmd::generate(Device::D, 200, 3);
        let cmd2 = ReadCmd::generate(Device::M, 200, 8);

        let res1: Result<Vec<u16>> = cmd1.decode(&[0xad, 0xde, 0xef, 0xbe, 0xff, 0x00]);
        let res2: Result<Vec<bool>> = cmd2.decode(&[0x11, 0x01, 0x11, 0x11]);

        assert!(res1.is_ok());
        assert!(res2.is_ok());
    }

    #[test]
    fn test_write_encode() {
        let (mut buf1, mut buf2) = (Vec::new(), Vec::new());

        WriteCmd::generate(Device::D, 200, 2, [0xde, 0xad, 0xbe, 0xef].to_vec()).encode(&mut buf1);
        WriteCmd::generate(Device::M, 200, 4, [0x11, 0x01].to_vec()).encode(&mut buf2);

        assert_eq!(
            buf1,
            [0x01, 0x14, 0x00, 0x00, 0xc8, 0x00, 0x00, 0xa8, 0x02, 0x00, 0xde, 0xad, 0xbe, 0xef]
        );
        assert_eq!(
            buf2,
            [0x01, 0x14, 0x01, 0x00, 0xc8, 0x00, 0x00, 0x90, 0x04, 0x00, 0x11, 0x01]
        );
    }
}
