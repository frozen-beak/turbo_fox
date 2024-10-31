use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Connect to the server at localhost on port 1234
    let mut stream = TcpStream::connect("127.0.0.1:1234").expect("Failed to connect to server");

    // Send a message to the server
    let message = b"hello";
    stream
        .write_all(message)
        .expect("Failed to write to server");

    // Read the server's response
    let mut buffer = [0; 64];
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("Server says: {}", response);
        }
        Ok(_) => {
            // No data read, server closed connection
            println!("Server closed the connection");
        }
        Err(e) => {
            eprintln!("Failed to read from server: {}", e);
        }
    }
}
