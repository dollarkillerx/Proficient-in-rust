use std::env;
use std::process;

pub struct Config {
    pub filename: String,
    pub key: String,
}

pub fn init_config() -> Config {
    let args: Vec<String> = env::args().collect();

    // middleware
    if args.len() != 3 {
        println!("you inputs: {:?}",args); // [ 程序名,参数1,参数2,...]
        println!("please input: ./cmd query_key filename");
        process::exit(1);
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Config{
        filename,
        key:query,
    }
}