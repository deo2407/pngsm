use crc::{Crc, CRC_32_ISO_HDLC};
use std::convert::TryFrom;
use std::fmt;

use crate::chunk_type::ChunkType;
use crate::{Result, Error};

#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
    pub crc: u32 
}

impl Chunk {
    fn calculate_crc(chunk_type: &ChunkType, chunk_data: &Vec<u8>) -> u32 {
        let mut message = chunk_type.bytes().to_vec(); 
        message.extend(chunk_data);

        const CRC_PNG: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        CRC_PNG.checksum(&message)
    }
    
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let checksum = Self::calculate_crc(&chunk_type, &data);
        Chunk {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc: checksum 
        }
    }
    
    pub fn length(&self) -> u32 {
        self.chunk_data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    } 

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.chunk_data.clone())
            .map_err(|_| "Invalid UTF8 sequence".into())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12 + self.length as usize);

        bytes.extend(self.length.to_be_bytes());
        bytes.extend(self.chunk_type.bytes());
        bytes.extend(&self.chunk_data);
        bytes.extend(&self.crc.to_be_bytes());

        bytes
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(raw_data: &[u8]) -> Result<Self> {
        if raw_data.len() < 12 {
            return Err("To short".into());
        }
        let length = u32::from_be_bytes(raw_data[0..4].try_into().unwrap());

        let chunk_type_bytes: [u8; 4] = raw_data[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let chunk_data = raw_data[8..8 + length as usize].to_vec();

        let crc_start = 8 + length as usize;
        let crc_end = crc_start + 4;
        let crc_bytes: [u8; 4] = raw_data[crc_start..crc_end].try_into().unwrap();
        let crc = u32::from_be_bytes(crc_bytes);

        let actual_crc = Self::calculate_crc(&chunk_type, &chunk_data);

        if actual_crc != crc {
            return Err(format!("CRC mismatch: expected {actual_crc}, got {crc}").into());
        }

        Ok(Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}
