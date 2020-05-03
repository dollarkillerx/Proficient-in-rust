use std::io;
use std::fs::File;
use std::io::{Read, Write};
use std::env;

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

pub fn get_env(ex: &str) -> Option<String> {
    let mut result:Option<String> = None;
    match env::var(ex) {
        Ok(t) => {
            result = Option::from(t.clone());
        }
        Err(e) => {},
    };

    result
}

pub fn search<'a>(data: &'a str,key: &str) -> Vec<&'a str> {
    let mut resp = Vec::new();
    for i in data.lines() {
        if i.contains(key) {
            resp.push(i)
        }
    }

    resp
}