use std::fmt;

#[derive(Debug)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Debug => write!(f, "{}", "DEBUG"),
            Level::Info => write!(f, "{}", "INFO"),
            Level::Warn => write!(f, "{}", "WARN"),
            Level::Error => write!(f, "{}", "ERROR"),
            Level::Fatal => write!(f, "{}", "FATAL"),
        }
    }
}
