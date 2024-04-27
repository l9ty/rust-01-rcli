use std::{fs, io};

use anyhow::Ok;

pub fn open_reader(infile: &str) -> anyhow::Result<Box<dyn io::Read>> {
    if infile == "-" {
        Ok(Box::new(io::stdin()) as Box<dyn io::Read>)
    } else {
        Ok(Box::new(fs::File::open(infile)?) as Box<dyn io::Read>)
    }
}

pub fn reader_content(infile: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader = open_reader(infile)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
