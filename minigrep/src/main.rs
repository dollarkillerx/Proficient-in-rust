use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // middleware
    if args.len() != 3 {
        println!("you inputs: {:?}",args); // [ 程序名,参数1,参数2,...]
        panic!("please input: ./cmd query_key filename");
    }

    let query = &args[1];
    let filename = &args[2];

    println!("query key: {} filename: {}",query,filename);
}