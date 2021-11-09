use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();
    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let resp = format!(
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
