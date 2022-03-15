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

    let get = b"GET / HTTP/1.1\r\n";
    let response: String;
    if buffer.starts_with(get) { // If buffer starts with expected string send index
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        // TODO: Handle potential error
        response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents,
        );
    } else { // If buffer is something we don't expect send 404 page
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        
        response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents,
        );
    }

    // TODO: Handle stream unwrap errors as well as flush
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}