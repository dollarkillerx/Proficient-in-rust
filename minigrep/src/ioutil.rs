use std::io;
use std::fs::File;
use std::io::{Read, Write};

pub fn read_file(filepath: String) -> Result<String,io::Error> {
    let mut result = String::new();

    File::open(filepath)?.
        read_to_string(&mut result)?;

    Ok(result)
}

pub fn write_file(filepath: String,body: String) -> Option<bool> {
    let mut fi = match File::create(filepath) {
        Err(e) => return None,
        Ok(f) => f,
    };

    match fi.write_all(body.as_bytes()) {
        Err(_) => return None,
        Ok(_) => Some(true),
    }
}