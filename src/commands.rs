use core::str;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::args::{Cli, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::Result;

pub fn run() -> Result<()> {
    let cli = Cli::build()?;

    match cli {
        Cli::Encode(args) => encode(&args), 
        Cli::Decode(args) => decode(&args), 
        Cli::Remove(args) => remove(&args), 
        Cli::Print(args) => print_chunks(&args), 
    }
}

fn encode(args: &EncodeArgs) -> Result<()> {
    let mut png = read_png(&args.file_path)?;
    let chunk = Chunk::new(args.chunk_type.clone(), args.message.clone().into());
    png.append_chunk(chunk);
    
    write_png(&png, &args.file_path)?;
    Ok(())
}

fn decode(args: &DecodeArgs) -> Result<()> {
    let mut png = read_png(&args.file_path)?;
    png.print_chunk(str::from_utf8(&args.chunk_type.bytes())?);

    Ok(())
}

fn remove(args: &RemoveArgs) -> Result<()> {
    let mut png = read_png(&args.file_path)?;
    let _ = png.remove_first_chunk(str::from_utf8(&args.chunk_type.bytes())?);
    write_png(&png, &args.file_path)?;
    
    Ok(())
}

fn print_chunks(args: &PrintArgs) -> Result<()> {
    let mut png = read_png(&args.file_path)?;
    println!("{:#?}", &png.chunks);

    Ok(())
}

fn read_png(file_path: &PathBuf) -> Result<Png> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to parse PNG");

    Ok(Png::try_from(contents.as_slice())?) 
}

fn write_png(png: &Png, file_path: &PathBuf) -> Result<()> {
    let mut file = File::create(file_path)?;
    let contents = png.as_bytes();

    file.write_all(&contents)?;
    Ok(())
}
