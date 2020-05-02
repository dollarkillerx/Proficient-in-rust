use std::fs::File;
use hellorust::*;
// use easy_file::EasyFile;
// use hellorust::EasyFile;

fn main() {
    // test1();
    // test2();
    test3();
}

fn test3() {
    let c = EasyFile::new();
    let data = match c.read_file("src/main.rs".parse().unwrap()) {
        Ok(msg) => msg,
        Err(e) => panic!("err: {:#?}",e),
    };

    println!("Data: {}",data);
}

fn test2() {
    let f = File::open("main.rs").expect("open file error");
    // let f = match File::open("main.rs") {
    //     Ok(c) => c ,
    //     Err(e) => panic!("err: {:#?}",e),
    // };
}

fn test1() {
    let f = File::open("src/main.rs");
    let r = match f {
        Err(e) => panic!("err: {:#?}",e),
        Ok(file) => file,
    };
}