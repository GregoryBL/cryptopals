//! Wrapper class for a Vec<u8> for crypto operations and conformances
use std::ops;
use std;
use super::hex::{
    HexError, 
    FromHexString,
    ToHexString,
};

#[derive(Debug)]
pub struct Buffer(pub Vec<u8>);

impl Buffer {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn from_string(str: &str) -> Self {
        Buffer(str.as_bytes().to_vec())
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match std::str::from_utf8(&self.0) {
            Ok(v) => v,
            Err(e) => { eprintln!("  Error displaying buffer: {}", e); "?" },
        };
        write!(f, "{}", as_string)
    }
}

impl ops::BitXor for Buffer {
    type Output = Self;
    fn bitxor(self, Buffer(rhs): Self) -> Self::Output {
        let Buffer(lhs) = self;
        Self(lhs.iter().zip(rhs.iter()).map( |(first, second)| {
            first ^ second
        }).collect())
    }
}

impl ops::BitXor for &Buffer {
    type Output = Buffer;
    fn bitxor(self, Buffer(rhs): Self) -> Self::Output {
        let Buffer(lhs) = self;
        Buffer(lhs.iter().zip(rhs.iter()).map( |(first, second)| {
            first ^ second
        }).collect())
    }
}

impl<'a> IntoIterator for &'a Buffer {
    type Item = u8;
    type IntoIter = BufferIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        BufferIter {
            buffer: self,
            index: 0
        }
    }
}

pub struct BufferIter<'a> {
    buffer: &'a Buffer,
    index: usize,
}

impl std::iter::Iterator for BufferIter<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.len() {
            return None
        };
        let result = self.buffer.0[self.index];
        self.index += 1;
        Some(result)
    }
}

fn char_to_oct(c: u8, idx: usize) -> Result<u8, HexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => {
            Err(HexError::InvalidHexCharacter {
                c: c as char,
                index: idx,
            })
        }
    }
}

impl FromHexString for Buffer {
    type Error = HexError;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, HexError> {
        let hex = hex.as_ref();
        if hex.len() % 2 != 0 {
            return Err(HexError::OddLength);
        }

        Ok(Buffer(
            hex.chunks(2).enumerate().map( |(i, pair)| {
                // first four bits, shift, second 4 bits
                Ok(char_to_oct(pair[0], i * 2)? << 4 | char_to_oct(pair[1], i * 2 + 1)?)
            }).collect::<Result<Vec<u8>, HexError>>()?
        ))
    }
}

fn oct_to_char(o: u8, idx: usize) -> Result<u8, HexError> {
    match o {
        0..=9 => Ok(b'0' + o),
        10..=15 => Ok(b'a' + o - 10),
        _ => {
            Err(HexError::InvalidHexCharacter {
                c: o as char,
                index: idx,
            })
        }
    }
}

impl ToHexString for Buffer {
    type Error = HexError;
    fn to_hex(&self) -> Result<String, Self::Error> {
        let vals = self.0.iter().fold(Vec::<u8>::new(), | mut acc, val | {
            let first = oct_to_char(val >> 4, 0).unwrap();
            let second = oct_to_char(val % 16, 0).unwrap();
            acc.push(first);
            acc.push(second);
            acc
        });
        match String::from_utf8(vals) {
            Ok(val) => Ok(val),
            Err(_) => Err(HexError::InvalidStringLength),
        }
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
