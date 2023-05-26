use std::io::prelude::*;
use std::net::TcpListener;
use std::string::String;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut buffer = [0; 96];

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Connection address: {:?}", stream.peer_addr()?);

        stream.read(&mut buffer)?;
        let string = String::from_utf8_lossy(&buffer);
        let string = string.replace("\0", "");
        println!("Received: {:?}", string);
    }

    Ok(())
}

// use std::io::{BufRead, BufReader};
// use std::{
//     net::{TcpListener, TcpStream},
//     thread,
// };

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         thread::spawn(move || {
//             handle_stream(stream);
//         });
//     }
// }

// fn handle_stream(stream: TcpStream) {
//     let mut reader = BufReader::new(stream);
//     let mut message = String::new();
//     reader.read_line(&mut message).unwrap();
//     println!("{}", message);
// }
