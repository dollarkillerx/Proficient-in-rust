use minigrep::*;
use std::process;

fn main() {
    // init
    let config = config::init_config();
    let data :String;
    match ioutil::read_file(config.filename) {
        Ok(t) => {
            data = t
        },
        Err(e) => {
            println!("Open File Err: {}",e);
            process::exit(1);
        }
    }

    // search
    let resp = ioutil::search(&data,&config.key);

    for i in resp {
        println!("find: {}",i);
    }
}

