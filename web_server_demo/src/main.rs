use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7880").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0;1024];
    
    stream.read(&mut buf).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buf[..]));

    let contents = fs::read_to_string("static/hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
// 