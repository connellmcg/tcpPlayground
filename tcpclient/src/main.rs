use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;


fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    // write hello to the server
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    // read the bytes recieved from the server
    stream.read(&mut buffer).unwrap();
    // print out what we recieved - we need to convert
    // the raw bytes to utf8 to read them
    println!("Got response from server {:?}",
        str::from_utf8(&buffer).unwrap()
    );

}
