use crate::utils;
use std::io::ErrorKind;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    text: String,
    location: Option<utils::CodeLoc>,
    ality: Ality,
}

#[derive(Debug)]
pub enum Ality {
    Fatal,
    Retry,
}

impl Ality {
    pub fn is_fatal(&self) -> bool {
        match self {
            Self::Fatal => true,
            _ => false,
        }
    }
}

impl Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str: String = self.text.clone();

        if self.ality.is_fatal() {
            str = format!("fatal: {}", str);
        }

        if let Some(loc) = self.location.clone() {
            str = format!("[{}]: {}", loc, str);
        }

        write!(f, "{}", str)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let text: String = e.to_string();
        let location: Option<utils::CodeLoc> = None;

        use std::io::ErrorKind;
        use Ality::*;
        let ality: Ality = match e.kind() {
            ErrorKind::NotFound => Fatal,
            ErrorKind::PermissionDenied => Fatal,
            ErrorKind::ConnectionRefused => Retry,
            ErrorKind::ConnectionReset => Retry,
            ErrorKind::ConnectionAborted => Retry,
            ErrorKind::NotConnected => Retry,
            ErrorKind::AddrInUse => Fatal,
            ErrorKind::AddrNotAvailable => Fatal,
            ErrorKind::BrokenPipe => Fatal,
            ErrorKind::AlreadyExists => Retry,
            ErrorKind::WouldBlock => Fatal,
            ErrorKind::InvalidInput => Retry,
            ErrorKind::InvalidData => Fatal,
            ErrorKind::TimedOut => Retry,
            ErrorKind::WriteZero => Retry,
            ErrorKind::Interrupted => Retry,
            ErrorKind::Other => Retry,
            ErrorKind::UnexpectedEof => Fatal,
            ErrorKind::Unsupported => Fatal,
            ErrorKind::OutOfMemory => Retry,

            _ => Fatal,
        };

        Self {
            text,
            location,
            ality
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        let text: String = e.to_string();
        let location: Option<utils::CodeLoc> = None;

        Self {
            text, location,
            ality: Ality::Fatal
        }
    }
}

impl From<pico_args::Error> for Error {
    fn from(e: pico_args::Error) -> Self {
        let text: String = e.to_string();
        let location: Option<utils::CodeLoc> = None;

        Self {
            text, location,
            ality: Ality::Fatal,
        }
    }
}
