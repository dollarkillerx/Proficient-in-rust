pub mod ioutil;
pub mod config;


#[cfg(test)]
mod test {
    use super::*;
    use std::env;

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
    #[test]
    fn get_os() {
        match ioutil::get_env("HOM") {
            Some(t) => println!("t: {}",t),
            None =>  println!("is null"),
        }
    }
    #[test]
    fn spl() {
        // let novel = String::from("Call me Ishmael. Some years ago ...");
        // let first_sentence = novel.split('.')
        //     .next()
        //     .expect("could not find a '.'");
        // println!("c: {}",first_sentence);
        // let key = "GOPATH";
        // match env::var(key) {
        //      Ok(val) => println!("{}: {:?}", key, val),
        //      Err(e) => println!("couldn't interpret {}: {}", key, e),
        // }

    }
}