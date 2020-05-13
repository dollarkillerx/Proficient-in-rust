use hyper::{Client,Uri};
use std::env;
use futures::compat::Future01CompatExt;

pub async fn client() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("please input get http addr");
        std::process::exit(1);
    }
    let url = args[1].clone().parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).compat().await;

    println!("Request: {:?}",res);
}