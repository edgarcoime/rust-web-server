use std::net::{TcpListener, TcpStream};
use std::io::prelude;

fn main() {
    // TODO: Handle error case gracefully
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // TODO: Handle error
        let stream = stream.unwrap();
        println!("Connection established");
    }
}