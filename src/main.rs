use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn redirect_handler(mut stream: TcpStream, redirect_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    match stream.peer_addr() {
        Ok(addr) => println!("Connection from {}", addr),
        Err(_) => println!("Connection from unknown peer"),
    }

    write!(stream, "HTTP/1.1 307 Temporary Redirect\nConnection: keep-alive\nLocation: {}\r\n\r\n", redirect_address)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("ADDR").expect("failed to get ADDR env variable");
    let redirect_address = std::env::var("REDIRECT_URI").expect("failed to get REDIRECT_URI env variable");

    let listener = TcpListener::bind(&addr)?;
    listener.set_ttl(100)?;

    for stream in listener.incoming() {
        redirect_handler(stream?, &redirect_address)?;
    }

    Ok(())
}
