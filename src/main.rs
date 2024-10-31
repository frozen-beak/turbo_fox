use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").expect("Failed to bind address");

    listener
        .set_nonblocking(true)
        .expect("Cannot set on-blocking!");

    println!("Listening on port: 1234");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream);
            }
            // Non-blocking accept failed - wait a bit
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => {
                println!("Failed to connect! Error: {e}")
            }
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 64];

    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            let received = String::from_utf8_lossy(&buffer[..n]);
            println!("Client says: {}", received);

            // Reply to the client
            let response = b"world";
            if let Err(e) = stream.write_all(response) {
                eprintln!("Failed to send response: {}", e);
            }
        }
        Ok(_) => {
            println!("Connection closed by client");
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
        }
    }
}
