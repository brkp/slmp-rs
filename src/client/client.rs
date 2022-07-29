use std::time::Duration;

/// TODO: docs
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
