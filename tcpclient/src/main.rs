use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!("Response from server {:?}", from_utf8(&buffer).unwrap());
}
