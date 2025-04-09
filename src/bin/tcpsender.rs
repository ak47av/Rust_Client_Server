use std::io::prelude::*;
use std::net::TcpStream;
use tcpexample::Client::Client;

fn main() -> std::io::Result<()> {
    let serverIP = String::from("127.0.0.1");
    Client::new(serverIP);
    Ok(())
}