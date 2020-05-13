use {
    hyper::{
        // create http server
        Body,Request,Response,Server,
        // 该函数将闭包转换为实现了Hyper的Service trait的future
        // 它是从通用Request到Response的异步函数。
        service::service_fn,
        // 使用Hyper运行时可以运行future到完成的函数。
        rt::run,
    },

    futures::{
        // futures 0.1版本的一个扩展trait，添加了'.compat()'方法
        // 允许我们在0.1版本的futures中使用'.await'语法
        compat::Future01CompatExt,
        // 扩展trait在futures上提供了额外的方法在
        // `FutureExt` 添加了适用于所有futures的方法,
        // `TryFutureExt` 给futures添加了可以放回‘Result’类型的方法
        future::{FutureExt,TryFutureExt},
    },
    std::net::SocketAddr,
    std::env,
};
use std::str::FromStr;

async fn server_req(req: Request<Body>) -> Result<Response<Body>,hyper::Error> {
    println!("request at {:?}",req.uri());
    Ok(Response::new(Body::from("Hello World!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listen addr: {}",addr);
    let server_future = Server::bind(&addr)
        .serve(|| service_fn(|req|
            server_req(req).boxed().compat()
        ));
    if let Err(e) = server_future.compat().await {
        eprintln!("server err: {}",e);
    }
}

pub fn httpserver() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("please input ./xxx 0.0.0.0:8081");
        std::process::exit(1);
    }
    let addr = SocketAddr::from_str(args[1].clone().as_str()).unwrap();

    let futures_032_future = run_server(addr);
    let futures_01_future = futures_032_future.unit_error().boxed().compat();
    run(futures_01_future);
}