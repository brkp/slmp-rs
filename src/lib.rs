#![allow(
    dead_code,
    unused_imports,
    unused_variables)]

//! TODO: docs

mod types;
mod client;
mod command;

pub use client::client::Config;
pub use types::{device::{Device, DeviceType}, end_code::SLMPEndCode};

use std::{io, fmt};

/// TODO: docs
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SLMPError(SLMPEndCode),
    CustomError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(ref err) => write!(f, "I/O error: {}", err),
            SLMPError(err) => write!(f, "SLMP error: {}", err),
            CustomError(err) => write!(f, "slmp-rs error: {}", err)
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::Io(ref err) => Some(err),
            _ => None,
        }
    }

    fn description(&self) -> &str {
        use Error::*;

        match self {
            Io(_) => "I/O error",
            SLMPError(_) => "SLMP error",
            CustomError(_) => "slmp-rs error",
        }
    }
}

/// TODO: docs
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

    #[test]
    fn it_works() {
        assert_eq!(SLMPEndCode::from_u16(0x0000), Some(SLMPEndCode::Success));
    }
}
