pub mod device;

type CmdFn = Box<dyn Fn(&mut Vec<u8>) -> usize>;
