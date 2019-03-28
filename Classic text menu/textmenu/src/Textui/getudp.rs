extern crate utp;
use utp::UtpStream;
use std::io::Write;

pub fn get(){
  match mode {
        Mode::Server => {
            // Create a listening stream
            let mut stream = UtpStream::bind("127.0.0.1:8080").expect("Error binding stream");
            let mut writer = stdout();
            let _ = writeln!(&mut stderr(), "Serving on {}", addr);

            // Create a reasonably sized buffer
            let mut payload = vec![0; 1024 * 1024];

            // Wait for a new connection and print the received data to stdout.
            // Reading and printing chunks like this feels more interactive than trying to read
            // everything with `read_to_end` and avoids resizing the buffer multiple times.
            loop {
                match stream.read(&mut payload) {
                    Ok(0) => break,
                    Ok(read) => writer.write(&payload[..read]).expect("Error writing to stdout"),
                    Err(e) => panic!("{}", e)
                };
            }
}
}