use core::str;
use std::fmt::{self, Display};

use crate::chunk::Chunk;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Png {
    pub header: [u8; 8],
    pub chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png {
            header: Self::STANDARD_HEADER,
            chunks
        }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn print_chunk(&mut self, chunk_type: &str) -> Result<()> {
        if let Some(chunk) = self.chunks.iter().find(|chunk| chunk.chunk_type.bytes() == chunk_type.as_bytes()) {
            println!("Chunk with type: {chunk_type} found: {}", str::from_utf8(&chunk.chunk_data)?);
            return Ok(());
        }
        Err(format!("Chunk with type {chunk_type} not found").into())

    }

    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        if let Some(index) = self.chunks.iter().position(|chunk| chunk.chunk_type.bytes() == chunk_type.as_bytes()) {
            return Ok(self.chunks.remove(index));
        }
        Err(format!("Chunk with type {chunk_type} not found").into())
    }

    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks.iter()
            .find(|chunk| chunk.chunk_type.bytes() == chunk_type.as_bytes())
    } 

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_bytes: &Vec<u8> = &self.chunks 
            .iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        Self::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect()
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let header: [u8; 8] = bytes[0..8].try_into()?;
        if header != Self::STANDARD_HEADER {
            return Err("Invalid PNG header".into())
        }

        let mut bytes = &bytes[8..];
        let mut chunks = Vec::new();

        while !bytes.is_empty() {
            let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
            let total_chunk_length = 12 + length as usize;
            
            if bytes.len() < total_chunk_length {
                return Err("Chunk length exceeds remaining data".into());
            }

            let chunk_bytes = &bytes[..total_chunk_length];
            let chunk = Chunk::try_from(chunk_bytes)?;
            chunks.push(chunk);

            bytes = &bytes[total_chunk_length..];
        } 

        Ok(Self::from_chunks(chunks))
    }
}
