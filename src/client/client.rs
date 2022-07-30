use crate::{Error, Result};

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Client configuration.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    port: u16,
    conn_timeout: Option<Duration>,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 12288,
            conn_timeout: None,
            read_timeout: None,
            write_timeout: None,
        }
    }
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Result<Self> {
        Self::new_with_cfg(addr, Config::default())
    }

    pub fn new_with_cfg(addr: &str, config: Config) -> Result<Self> {
        let stream = match config.conn_timeout {
            Some(time) => {
                let mut sock_addr = (addr, config.port).to_socket_addrs().unwrap();
                TcpStream::connect_timeout(&sock_addr.next().unwrap(), time)
            }
            None => TcpStream::connect((addr, config.port)),
        };

        match stream {
            Ok(s) => {
                s.set_read_timeout(config.read_timeout)?;
                s.set_write_timeout(config.write_timeout)?;
                s.set_nodelay(true)?;

                Ok(Self { stream: s })
            }
            Err(err) => Err(err.into()),
        }
    }
}
