use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;

use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    pub bytes: [u8; 4]
}

impl ChunkType { 
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    
    fn is_zero_fifth(byte: u8) -> bool {
        let mask = 1 << 5;
        byte & mask == 0
    }

    pub fn is_critical(&self) -> bool {
        Self::is_zero_fifth(self.bytes[0]) 
    }
    
    pub fn is_public(&self) -> bool {
        Self::is_zero_fifth(self.bytes[1]) 
    }
    
    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::is_zero_fifth(self.bytes[2]) 
    }
    
    pub fn is_safe_to_copy(&self) -> bool {
        !Self::is_zero_fifth(self.bytes[3]) 
    }

    pub fn is_valid(&self) -> bool {
        for byte in self.bytes.iter() {
            if !byte.is_ascii_alphabetic() || !self.is_reserved_bit_valid() { 
                return false;
            }    
        }
        true
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

impl FromStr for ChunkType {
    type Err = Error; 

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("A chunk should be 4 bytes".into());
        } 
        let mut byte_arr: [u8; 4] = [0; 4];

        for (index, byte) in s.bytes().enumerate() {
            if byte.is_ascii_alphabetic() {
                byte_arr[index] = byte;
            } else {
                return Err(format!("Tried to parse an invalid byte: {byte}", byte = byte).into());
            }
        }
        Ok(Self { bytes: byte_arr })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        for byte in bytes.iter() {
            if !byte.is_ascii_alphabetic() {
                return Err(format!("Tried to parse an invalid byte: {byte}", byte = byte).into());
            }
        }
        Ok(Self { bytes })
    }
}
