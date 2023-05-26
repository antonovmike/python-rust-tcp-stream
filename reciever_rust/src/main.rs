use std::io::prelude::*;
use std::net::TcpListener;
use std::string::String;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6006")?;
    let mut buffer = [0; 96];
    println!("Connection address: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let mut stream = stream?;

        stream.read(&mut buffer)?;
        let string = String::from_utf8_lossy(&buffer);
        let string = string.replace("\0", "");
        println!("Received: {:?}", string);
    }

    Ok(())
}
