use std::thread;
use std::time::Duration;

pub fn test1() {
    let s1 = thread::spawn(hello);
    thread::spawn(hello2);
    thread::spawn(||{
        thread::sleep(Duration::from_secs(1));
        panic!("Thread 3 Panic"); // 子线程Panic 主线程不会Panic
    });

    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    s1.join().unwrap();
    println!("Over");
}

fn hello2() {
    let mut i = 0;
    loop {
        if i < 1000 {
            i += 1;
            thread::sleep(Duration::from_millis(500));
            println!("S2 i: {}",i);
            continue;
        }
        break
    }
}

fn hello() {
    let mut i = 0;
    loop {
        if i < 1000 {
            i += 1;
            thread::sleep(Duration::from_millis(1));
            println!("S1 i: {}",i);
            continue;
        }
        break
    }
}