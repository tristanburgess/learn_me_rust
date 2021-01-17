use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use mt_web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buf[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "bad.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let resp = format!("{}\r\n{}\r\n\r\n{}", status_line, contents.len(), contents);

    //println!("Response: {}", resp);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.exec(|| {
            handle_connection(stream);
        });
    }
}
