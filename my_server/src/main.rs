use std::net::{TcpListener,TcpStream};
use std::io::{Write, Read};
use std::{fs,thread};

fn main() -> std::io::Result<()>{
    let listen = TcpListener::bind("0.0.0.0:8081")?;
    for conn in listen.incoming() {
        // match conn {
        //     Ok(conn) => {
        //         handel(conn);
        //     },
        //     Err(e) => panic!(e),
        // }

        // handel(conn?);

        thread::spawn(move || {
            handel(conn.unwrap());
        });
    }

    Ok(())
}

fn handel(mut conn: TcpStream) {
    // read msg
    println!("addr: {}",conn.peer_addr().unwrap().to_string());
    let mut buffer = [0;512];
    conn.read(&mut buffer).unwrap();
    println!("read msg: {}",String::from_utf8_lossy(&buffer[..]));

    let ik = b"GET / HTTP/1.1";
    if buffer.starts_with(ik) {
        // send html
        let html = fs::read_to_string("index.html").unwrap();
        let body = format!("HTTP/1.1 200 OK\r\n\r\n{}",html);
        conn.write(body.as_bytes()).unwrap();
        conn.flush().unwrap();
    }else {
        // send html
        let body = "HTTP/1.1 404 NOT FUND\r\n\r\n 404";
        conn.write(body.as_bytes()).unwrap();
        conn.flush().unwrap();
    }



    // send msg
    // match conn.write(String::from("Hello Rust Tcp Server").as_bytes()) {
    //     Ok(i) => println!("size: {}",i),
    //     Err(e) => println!("err: {}",e),
    // }
}

// const A:&'static str = "sasdsadsad";

