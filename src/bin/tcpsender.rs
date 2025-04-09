use std::io::prelude::*;
use std::net::TcpStream;
use tcpexample::Client::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let serverIP = String::from("127.0.0.1");
    let client = Client::new(serverIP);
    Ok(())
}