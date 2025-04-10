use std::net::{TcpStream};
use std::error::Error;
use std::io::{Read, Write};
use std::str;

use crate::DateTimeService;
use crate::Communication::Communication;

pub struct Client {
    stream: TcpStream,
}

impl Client {

    // Initializes the client
    pub fn new(serverIP: String) -> Result<Self, Box<dyn Error>> {
        let portNumber = 34254;
        let serverAddr = format!("{}:{}", serverIP, portNumber);

        // Connect to the server
        let mut st = TcpStream::connect(serverAddr)?;

        // Get the address of the server
        let peerAddr = st.peer_addr()?;
        let peerAddrString = peerAddr.to_string();
        println!("00. => Connected to Server: {}", peerAddrString);

        // Get the address of the local instance
        let localAddr = st.local_addr()?;
        let localAddrString = localAddr.to_string();
        println!("00. => From local: {}", localAddrString);

        Ok(Self{stream: st})
    }

    // Send a date command to the server 
    pub fn getDate(&mut self) {
        let theDateCommand = String::from("GetDate");
        println!("01. -? Sending Command {} to the server..", theDateCommand);
        // Send the Date Command
        let mut buffer = [0u8; 100];
        let length = Client::encode(theDateCommand, &mut buffer);
        self.send(&mut buffer);
        // Receive the Date and time
    }

}

// Implement the Communication Trait
impl Communication for Client {
    // send contents of the buffer over the stream
    fn send(&mut self, buf: &mut[u8]) -> Result<(), Box<dyn Error>>{
        let bytes = self.stream.write(buf)?;
        Ok(())
    }

    // receive contents from the stream and place them into the buffer
    fn receive(&mut self, buf: &mut[u8]) -> Result<usize, Box<dyn Error>> {
        let bytes_received = self.stream.read(buf)?;
        Ok(bytes_received)
    }
}

