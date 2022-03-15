use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    // TODO: Handle error case gracefully
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // TODO: Handle error
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // TODO: handle reading error
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";

    // TODO: Handle stream unwrap errors as well as flush
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}