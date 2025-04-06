use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;
    let data = [2];
    stream.write(b"This is year of Linux on Desktop")?;
    stream.read(&mut [0; 128])?;
    Ok(())
}