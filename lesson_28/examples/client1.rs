use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9527").unwrap();
    stream.write_all(b"hello world!").unwrap();
    let mut buf = [0u8; 17];
    stream.read_exact(&mut buf).unwrap();
    println!("data: {:?}", String::from_utf8_lossy(&buf));
}

