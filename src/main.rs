use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

use threadpool::ThreadPool;

extern crate threadpool;

fn main() {
    let streams = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in streams.incoming() {
        pool.execute(||{
            handle(stream.unwrap());
        })
    }
}

fn handle (mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf));
    let (stat, file) = if buf.starts_with(b"GET / ") {
        ("200 OK", "index.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string("./public/views/".to_owned()+ &file).unwrap();
    let res = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",stat, contents.len(), contents);
    stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap()
}