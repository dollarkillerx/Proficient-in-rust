use std::fs::File;
use std::io::Read;
use std::io;

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn main() {
    let c = "src/main.rs";
    match read_file(c) {
        Ok(data) => println!("data: {}",data),
        Err(e) => println!("err: {}",e),
    }
    println!("Hello, world!");
}
