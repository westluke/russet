use std::sync::mpsc::{SendError, TryRecvError};
use std::io;

use flexi_logger::{FlexiLoggerError};

#[derive(Debug)]
pub enum SetErrorKind {
    Io(io::Error),
    // TryFromInt(std::num::TryFromIntError),
    // Fmt,
    // SmallScreen,
    // LayoutOob,
    ThreadComm,
    Logging,
    Conversion,
    PanelOob,
}

#[derive(Debug)]
pub struct SetError {
    pub kind: SetErrorKind,
    pub msg: String
}

impl SetError {
    pub fn new(kind: SetErrorKind, msg: &str) -> Self {
        Self{ kind, msg: String::new() }
    }
}

impl From<io::Error> for SetError {
    fn from(err: io::Error) -> Self {
        Self::new(SetErrorKind::Io(err), "io error detected")
    }
}

// impl From<std::num::TryFromIntError> for SetError {
//     fn from(err: std::num::TryFromIntError) -> Self {
//         Self::new(SetErrorKind::TryFromInt(err), "tryfromint error detected")
//     }
// }

// impl From<std::fmt::Error> for SetError {
//     fn from(err: std::fmt::Error) -> Self {
//         Self::new(SetErrorKind::Fmt, "fmt error detected")
//     }
// }

impl<T> From<SendError<T>> for SetError {
    fn from(err: SendError<T>) -> Self {
        Self::new(SetErrorKind::ThreadComm, "send error detected")
    }
}

impl From<TryRecvError> for SetError {
    fn from(err: TryRecvError) -> Self {
        Self::new(SetErrorKind::ThreadComm, "recv error detected")
    }
}

 impl From<FlexiLoggerError> for SetError {
     fn from(err: FlexiLoggerError) -> Self {
         Self::new(SetErrorKind::Logging, "logging error detected")
     }
 }

