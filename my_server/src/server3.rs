use std::net::{TcpStream,TcpListener};
use std::io::{Write,Read};
use super::thread_pool;
use std::thread;
use std::time::Duration;

pub fn test_sever3() -> std::io::Result<()> {
    let listen = TcpListener::bind("0.0.0.0:8081")?;

    let c = thread_pool::ThreadPool::new(20);
    for conn in listen.incoming() {
        c.exec(||{
            web(conn.unwrap())
        });
    }

    Ok(())
}

fn web(mut conn: TcpStream) {
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

    thread::sleep(Duration::from_secs(3));
}