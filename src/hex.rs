use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HexError {
    InvalidHexCharacter {
        c: char,
        index: usize,
    },
    OddLength,
    InvalidStringLength,
}

impl error::Error for HexError {
    fn description(&self) -> &str {
        match *self {
            HexError::InvalidHexCharacter { .. } => "invalid character",
            HexError::OddLength => "odd number of digits",
            HexError::InvalidStringLength => "invalid string length",
        }
    }
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HexError::InvalidHexCharacter { c, index } =>
                write!(f, "Invalid character '{}' at position {}", c, index),
            HexError::OddLength => write!(f, "Hex string must have even length"),
            HexError::InvalidStringLength => write!(f, "Hex string length didn't match container length"),
        }
    }
}

pub trait FromHexString: Sized {
    type Error;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>;
}

pub trait ToHexString {
    type Error;
    fn to_hex(&self) -> Result<String, Self::Error>;
}


