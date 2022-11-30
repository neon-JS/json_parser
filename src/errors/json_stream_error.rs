use std::fmt;
use std::fmt::Formatter;

pub enum JsonStreamError {
    OutOfRange
}

impl fmt::Display for JsonStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JsonStreamError::OutOfRange => write!(f, "Could not consume stream: Out of range."),
        }
    }
}

impl fmt::Debug for JsonStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}