use std::alloc::handle_alloc_error;
use std::io::{Read, Write, BufRead};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::panic::Location;
use std::thread;
use std::time::Duration;
use json::{JsonValue, object};
use json::object::Object;
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
    split(String::from_utf8_lossy(&buffer[..]).to_string(), stream);
    return;
}

extern crate json;
fn split(http_content: String, mut stream: TcpStream) {
    let body_raw = http_content.split("\r\n").collect::<Vec<&str>>();
    let body = json::parse(body_raw[body_raw.len() - 1].trim_end_matches("\0")).unwrap();
    let cmd = body.to_owned()["cmd"].to_string();
    let out = shell(cmd);
    println!("{}", out);
    let contents = format!("{}", out);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\nAccess-Control-Allow-Credentials: true\r\nAccess-Control-Allow-Headers: Origin,No-Cache, X-Requested-With, If-Modified-Since, Pragma, Last-Modified, Cache-Control, Expires, Content-Type, X-E4M-With, token\r\nAccess-Control-Allow-Methods: POST, GET, OPTIONS, PUT, DELETE\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json;charset=utf-8\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    return;
}