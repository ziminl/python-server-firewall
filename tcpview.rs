





use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").expect("fail to bind ip");
    println!("1111");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let client_address = stream.peer_addr().expect("Failed to get client address");
                println!("Received connection from: {:?}", client_address);
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("stream failed");
    let response = "22222222\n";
    stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    stream.flush().expect("Failed to flush stream");
}






