






use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
const BLOCKED_IPS: [&str; 2] = ["1.1.1.1", "8.8.8.8"];
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind address");
    println!("waiting for response");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let client_address = stream.peer_addr().expect("Failed to get client address");
                println!("Received connection from: {:?}", client_address);
                if BLOCKED_IPS.contains(&client_address.ip().to_string().as_str()) {
                    println!("Blocked connection from: {:?}", client_address);
                    continue;
                }
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from stream");
    let response = "Hello, client!\n";
    stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    stream.flush().expect("Failed to flush stream");
}



