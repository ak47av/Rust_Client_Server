use tcpexample::Client::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let serverIP = String::from("127.0.0.1");
    let mut client = Client::new(serverIP)?;

    // Sends a command to the server
    client.getDate();

    Ok(())
}