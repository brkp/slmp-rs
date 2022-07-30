#![allow(
    dead_code,
    unused_imports,
    unused_variables)]

//! TODO: docs

/// TCP client related code.
pub mod client;
/// Functionality related to building/decoding SLMP commands.
pub mod command;
/// A set of SLMP related type definitons.
pub mod types;

use std::{fmt, io};
use crate::client::client::Config;
use crate::types::{
    cmd::Command,
    device::{Device, DeviceType},
    end_code::EndCode,
    header::Header,
};

/// TODO: docs
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidDevice,
    SLMPError(EndCode),
    CustomError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<&str> for Error {
    fn from(string: &str) -> Self {
        Self::CustomError(string.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(ref err) => write!(f, "I/O error: {}", err),
            InvalidDevice => write!(f, "InvalidDevice"),
            SLMPError(err) => write!(f, "SLMP error: {}", err),
            CustomError(err) => write!(f, "slmp-rs error: {}", err),
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
            InvalidDevice => "Invalid device specified",
            SLMPError(_) => "SLMP error",
            CustomError(_) => "slmp-rs error",
        }
    }
}

/// Type alias for `Result<T, slmp::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Convenience re-export of common members.
pub mod prelude {
    pub use crate::client::client::Config;
    pub use crate::types::{
        cmd::Command,
        device::{Device, DeviceType},
        end_code::EndCode,
        header::Header,
    };
}
