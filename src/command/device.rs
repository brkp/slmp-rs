use crate::command::CmdFn;
use crate::{Command, Device, DeviceType};

pub(crate) fn read(device: Device, addr: u32, count: u16) -> CmdFn {
    let cmd = Command::DeviceRead as u16;
    let sub = match device.get_type() {
        DeviceType::Bit => 0x0001u16,
        DeviceType::Word => 0x0000u16,
        DeviceType::DoubleWord => 0x0000u16,
    };

    let closure = move |buf: &mut Vec<u8>| {
        buf.extend_from_slice(&cmd.to_le_bytes());
        buf.extend_from_slice(&sub.to_le_bytes());
        buf.extend_from_slice(&addr.to_le_bytes()[..3]);
        buf.push(device as u8);
        buf.extend_from_slice(&count.to_le_bytes());

        10
    };

    Box::new(closure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let (mut buf1, mut buf2) = (Vec::new(), Vec::new());
        read(Device::D, 200, 8)(&mut buf1);
        read(Device::M, 200, 1)(&mut buf2);

        assert_eq!(buf1, [0x01, 0x04, 0x00, 0x00, 0xc8, 0x00, 0x00, 0xa8, 0x08, 0x00]);
        assert_eq!(buf2, [0x01, 0x04, 0x01, 0x00, 0xc8, 0x00, 0x00, 0x90, 0x01, 0x00]);
    }
}
