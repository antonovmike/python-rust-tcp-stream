use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        stream.write_all(b"Hello, world (Rust)")?;
        thread::sleep(Duration::from_secs(6));
    }
}
