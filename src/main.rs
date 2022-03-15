use std::fs;
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

    let contents = fs::read_to_string("index.html").unwrap();
    // TODO: Handle potential error
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
    );

    // TODO: Handle stream unwrap errors as well as flush
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}