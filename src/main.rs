use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];

    match stream.read(&mut buf) {
        Ok(_) => {
            let req = String::from_utf8_lossy(&buf);
            println!("handle_read called:\n{}", req);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 101 Switching Protocols
    Upgrade: websocket
    Connection: Upgrade
    Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

    match stream.write(response) {
        Ok(_) => println!("handle_write called: \nResponse sent"),
        Err(e) => println!("Failed to send response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}
