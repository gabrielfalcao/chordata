use crate::errors;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(name: &str) -> Result<Vec<u8>, errors::Error> {
    let mut f = File::open(name)?;
    let mut buffer: Vec<u8> = Vec::new();

    f.read(&mut buffer)?;

    Ok(buffer)
}
