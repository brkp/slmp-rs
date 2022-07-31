// TODO: figure out how to properly do the subcommand stuff
// right now they're literally hardcoded
// which would cause problems with different PLCs

pub mod device;
use crate::Result;

macro_rules! make_cmd {
    (struct $name:ident, $($fname:ident : $ftype:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname : $ftype),*
        }

        impl $name {
            fn generate($($fname : $ftype),*) -> Self {
                Self { $($fname),* }
            }
        }
    };
}
pub(crate) use make_cmd;

/// Every `*Cmd` needs to implement this in order for the `Request`
/// structure to be able to generate a request payload.
pub trait CmdEncode {
    fn encode(&self, buf: &mut Vec<u8>) -> usize;
}

/// Every `*Cmd` that needs to parse something from a `Response`
/// needs to implement this trait.
pub trait CmdDecode<T> {
    fn decode(&self, buf: &[u8]) -> Result<T>;
}
