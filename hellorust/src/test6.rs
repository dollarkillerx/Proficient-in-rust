use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
// use std::time::Duration;

pub fn test6() {
    let a = Arc::new(Mutex::new(0));
    let mut li = vec![];
    for _i in 0..100 {
        let ic = Arc::clone(&a);
        let th = thread::spawn(move || {
            *ic.lock().unwrap() += 1;
            // loop {
            //     thread::sleep(Duration::from_millis(1))
            // }
        });
        li.push(th);
    }

    for i in li {
        i.join().unwrap()
    }

    println!("Over: {}",*a.lock().unwrap())
}