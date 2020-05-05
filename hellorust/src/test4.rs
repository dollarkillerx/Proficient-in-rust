use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test4() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    loop {
        match rx.recv() {
            Ok(d) => println!("data: {}",d),
            Err(e) => {
                println!("err: {}",e);
                break;
            },
        }
    }

}