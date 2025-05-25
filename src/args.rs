use std::path::PathBuf;
use std::env::{self, Args};
use std::str::FromStr;

use crate::chunk_type::ChunkType;
use crate::Result;

#[derive(Debug)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String    
}

#[derive(Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType
}

#[derive(Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType
}

#[derive(Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf 
}

#[derive(Debug)]
pub enum Cli {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

impl Cli {
    pub fn build() -> Result<Cli> {
        let mut args = env::args();
        args.next();

        match args.next() {
            Some(arg) => Self::parse(arg.as_str(), &mut args),
            None => Err("No arguments provided".into())
        }
    }

    fn parse(arg_type: &str, args: &mut Args) -> Result<Cli> {
        match arg_type {
            "encode" => Self::parse_encode(args),
            "decode" => Self::parse_decode(args),
            "print" => Self::parse_print(args),
            "remove" => Self::parse_remove(args),
            _ => Err(format!("Unkown argument provided: {arg_type}").into())
        }
    }

    fn parse_encode(args: &mut Args) -> Result<Cli> {
        let file_path = match args.next() {
            Some(path) => PathBuf::from(path),
            None => return Err("File path not provided!".into())
        };
        let chunk_type = match args.next() {
            Some(value) => ChunkType::from_str(&value)?,
            None => return Err("No chunk type provided".into())
        };
        let message = match args.next() {
            Some(value) => value,
            None => return Err("No message provided".into())
        };
        Ok(Cli::Encode(EncodeArgs {
            file_path,
            chunk_type,
            message
        }))
    }
    fn parse_decode(args: &mut Args) -> Result<Cli> {
        let file_path = match args.next() {
            Some(path) => PathBuf::from(path),
            None => return Err("File path not provided!".into())
        };
        let chunk_type = match args.next() {
            Some(value) => ChunkType::from_str(&value)?,
            None => return Err("No chunk type provided".into())
        };
        
        Ok(Cli::Decode(DecodeArgs {
            file_path,
            chunk_type,
        }))
    }
    fn parse_remove(args: &mut Args) -> Result<Cli> {
        let file_path = match args.next() {
            Some(path) => PathBuf::from(path),
            None => return Err("File path not provided!".into())
        };
        let chunk_type = match args.next() {
            Some(value) => ChunkType::from_str(&value)?,
            None => return Err("No chunk type provided".into())
        };
        
        Ok(Cli::Remove(RemoveArgs {
            file_path,
            chunk_type,
        }))
    } 
    fn parse_print(args: &mut Args) -> Result<Cli> {
        let file_path = match args.next() {
            Some(path) => PathBuf::from(path),
            None => return Err("File path not provided!".into())
        };
        
        Ok(Cli::Print(PrintArgs {
            file_path,
        }))
    } 
}