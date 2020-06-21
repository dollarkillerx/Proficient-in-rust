// use futures::{executor, Future};
use futures::{executor};

async fn foo(x: &u8) -> u8 {
    *x
}

// fn foo_expand<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
//     async {
//         *x
//     }
// }

async fn async_func1() {
    println!("async function +++ !");
}

async fn async_fun2() {
    println!("async function 2 +++ !");
}

async fn async_main() {
    let f1 = async_func1();
    let f2 = async_fun2();

    let f = async move {
        f1.await;
        f2.await;
    };

    futures::join!(f,async_func3());
}

async fn async_func3() {
    println!("aaaaa")
}

fn main() {
    test2();
}

fn test1() {
    let x = 5;
    let f = foo(&x);
    executor::block_on(f);
    println!("Hello, world!");
}

fn test2() {
    executor::block_on(async_main());
    println!("Hello, world!");
}