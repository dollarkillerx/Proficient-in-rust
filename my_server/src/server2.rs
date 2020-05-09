use std::net::{TcpListener,TcpStream};
use std::{thread};
use std::io::{Read,Write};
use std::sync::{Arc,Mutex};
use std::time::Duration;

pub fn test_server2() -> std::io::Result<()> {
    let listen = TcpListener::bind("0.0.0.0:8081")?;
    let limit = Arc::new(Mutex::new(0));

    for conn in listen.incoming() {
        loop {
            let mut c = limit.lock().unwrap();
            if *c > 3 {   // 设置 limit 长度
                println!("水池满了 等待...");
                thread::sleep(Duration::from_millis(50));
                continue;
            }else {
                *c += 1;
                let c = Arc::clone(&limit);
                thread::spawn( move ||{
                    handle_func(conn.unwrap(),c);
                });
                break;
            }

        }
    }

    Ok(())
}

fn handle_func(mut conn: TcpStream,limit: Arc<Mutex<i32>>) {
    // read msg
    let mut buffer = [0;512];
    conn.read(&mut buffer).unwrap();
    println!("New Http Client Addr: {}",conn.peer_addr().unwrap());
    println!("{}",String::from_utf8_lossy(&buffer[..]));
    println!();
    // write html
    let base_handel = b"GET /hello HTTP/1.1";
    if buffer.starts_with(base_handel) {
        // hello
        let resp_data = b"HTTP/1.1 200 OK\r\n\r\n HELLO WORLD";
        conn.write(resp_data).unwrap();
    }else {
        let resp_data = b"HTTP/1.1 404 NOT FUND\r\n\r\n 404";
        conn.write(resp_data).unwrap();
    }
    thread::sleep(Duration::from_secs(20));
    *limit.lock().unwrap() += -1;
}