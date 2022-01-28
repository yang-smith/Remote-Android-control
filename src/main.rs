use std::alloc::handle_alloc_error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use web_server::net::net_threadpool::ThreadPool;


fn main() {

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

