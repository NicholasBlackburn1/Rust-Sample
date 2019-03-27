
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
         
        }
    }


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let i = 0;
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("src/lib/hello.html").unwrap();
    let http = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);


    stream.read(&mut buffer).unwrap();
    stream.write(http.as_bytes()).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.flush().unwrap();
}