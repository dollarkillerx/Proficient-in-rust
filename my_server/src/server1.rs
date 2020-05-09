use std::net::{TcpListener,TcpStream};
use std::{thread};
use std::io::{Read,Write};

pub fn test_server1() -> std::io::Result<()> {
    let listen = TcpListener::bind("0.0.0.0:8081")?;

    for conn in listen.incoming() {
        thread::spawn(move ||{
            handle_func(conn.unwrap())
        });
    }

    Ok(())
}

fn handle_func(mut conn: TcpStream) {
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
}