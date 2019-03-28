extern crate utp;
use utp::UtpStream;
use std::io::Write;

pub fn send(){

  // Connect to an hypothetical local server running on port 8080
    let addr = "127.0.0.1:8080";
    let mut stream = UtpStream::connect(addr).expect("Error connecting to remote peer");

    // Send a string
    stream.write("Test".as_bytes()).expect("Write failed");

    // Close the stream
    stream.close().expect("Error closing connection");
}