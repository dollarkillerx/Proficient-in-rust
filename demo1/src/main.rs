use async_std::task;

async fn hello() {
    // println!("Hello Async Rust");
    hello_sync();

    // 所以为了从异步进入同步，我们不需要做任何额外的设置-只需调用同步函数就这么简单，
    //除了...额...我们需要仔细关注需要花费很长时间的同步函数。孔子说在异步世界调用同步函数一定要三思。

    // 调用同步代码 异步就进入了同步
}

// sync hello
fn hello_sync() {
    println!("Hello Sync Async Rust");
}

fn test1() {
    task::block_on(hello());
    // task::
}

fn main() {
    test1();
    println!("Hello, world!");
}
