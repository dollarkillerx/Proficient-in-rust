use std::sync::mpsc;
use std::thread;

pub fn test3() {
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");

        match tx.send(val) {
            Ok(_) => println!("send success"),
            Err(e) => println!("send err: {}",e),
        }
        // 发送 移交所有权
    });


    let received = rx.recv().unwrap();
    // 接受数据 获得所有权   recv() 如果 生产者全部死了 就会 返回错误  反之就会阻塞一直等待数据


    println!("Get: {}",received);

}
