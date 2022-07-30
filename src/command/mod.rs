pub mod device;

/// A closure with the type `CmdFn` is returned from each command generator
/// function so that some other function can call this closure to build the
/// command data in-place, without knowing anything about the command.
pub type CmdFn = Box<dyn Fn(&mut Vec<u8>) -> usize>;
