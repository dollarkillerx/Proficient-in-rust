use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test5() {
    let (tx,rx) = mpsc::channel();
    // 增加一个生产者
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        for i in 0..99 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        for i in 100..999 {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in rx {
        println!("Resp: {}",i);
    }
}