use std::alloc::handle_alloc_error;
use std::io::{Read, Write, BufRead};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::panic::Location;
use std::thread;
use std::time::Duration;
use web_server::net::net_threadpool::ThreadPool;
use web_server::command::command::shell;
use std::string;

fn main() {

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(1000) {
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
    let out = split(String::from_utf8_lossy(&buffer[..]).to_string(), stream);

    return;
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./html/404.html")
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

extern crate json;
fn split(str_get: String, mut stream: TcpStream) -> String {
    let take: Vec<&str> = str_get.split(" HTTP/1.1").collect();
    let head = take[0].to_string();
    let take: Vec<&str> = head.split("GET /").collect();
    let head = take[1].to_string();
    println!("{}", head);
    let cmd = head.replace("/", " ");
    println!("{}",cmd);
    let out = shell(cmd.to_string());
    let out = json::stringify(out.split("\n").collect::<Vec<&str>>());
    println!("{}", out);

    let content_start = fs::read_to_string("./html/start.html").unwrap();
    let content_end = fs::read_to_string("./html/end.html").unwrap();
    let contents = format!("{}", out);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\nAccess-Control-Allow-Credentials: true\r\nAccess-Control-Allow-Headers: Origin,No-Cache, X-Requested-With, If-Modified-Since, Pragma, Last-Modified, Cache-Control, Expires, Content-Type, X-E4M-With, token\r\nAccess-Control-Allow-Methods: POST, GET, OPTIONS, PUT, DELETE\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json;charset=utf-8\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    return cmd.to_string();
}