// TODO: figure out how to properly do the subcommand stuff
// right now they're literally hardcoded
// which would cause problems with different PLCs

pub mod device;
use crate::Result;

macro_rules! command {
    ($name:ident, $($fname:ident : $ftype:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname : $ftype),*
        }

        impl $name {
            pub fn new($($fname : $ftype),*) -> Self {
                Self { $($fname),* }
            }
        }
    };
}
pub(crate) use command;

/// Every `*Cmd` needs to implement this in order for the `Request`
/// structure to be able to generate a request payload.
pub trait Cmd<T> {
    fn decode(buf: &mut Vec<u8>) -> Result<T>;
    fn encode(&self, buf: &mut Vec<u8>);
}
