use std::net::TcpListener;
//Tcp Stream implements read and write traits, so including this to bring it into scope
use std::io::{Read, Write};

fn main() {
    // Initialise listener on port 3000
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000!");
    // listen for new connections
    for stream in connection_listener.incoming() {
        // make the stream mutable so you can read and write to it
        let mut stream = stream.unwrap();
        println!("Connection Established");

        let mut buffer = [0; 1024];
        // read from incoming stream
        stream.read(&mut buffer).unwrap();
        // write back to client
        stream.write(&mut buffer).unwrap();
    }
}
