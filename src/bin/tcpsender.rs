use std::error::Error;
use tcpexample::Client::Client;

fn main() -> Result<(), Box<dyn Error>>{
    let serverIP = String::from("127.0.0.1");
    let mut client = Client::new(serverIP)?;
    let mut object = 745;
    client.send(&object)?;
    object = 943;
    client.send(&object)?;
    Ok(())
}