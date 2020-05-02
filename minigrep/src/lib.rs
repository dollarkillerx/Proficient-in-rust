pub mod ioutil;


#[cfg(test)]
mod test {
    use crate::ioutil;
    #[test]
    fn write() {
        match ioutil::write_file("a.txt".to_string(),"hello world".to_string()) {
            Some(b) => println!("Success: {}",b),
            None => panic!("write file error"),
        }
    }
    #[test]
    fn read_file() {
        match ioutil::read_file("src/ioutil.rs".to_string()) {
            Ok(msg) => println!("msg: {}",msg),
            Err(e) => panic!(e),
        }
    }
}