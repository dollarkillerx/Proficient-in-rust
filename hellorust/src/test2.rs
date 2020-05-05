use std::thread;
use std::time::Duration;

pub fn test2() {
    let ips = vec!["0.0.0.0","1.1.1.1","2.2.2.2"];
    let a = thread::spawn(move ||{
        // for i in ips {
        //     println!("Ic: {}",i);
        // }
        thread::sleep(Duration::from_secs(1));
        println!("ips: {:?}",ips);
        // rust 不知道ips 的生命周期长度
        // 无法保证ips 始终有效   所有需要移交所有权给它
    });

    a.join().unwrap()
}