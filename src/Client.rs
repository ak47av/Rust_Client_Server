use std::net::{TcpStream};
use std::error::Error;
use std::io::{Read, Write};
use crate::DateTimeService;

pub struct Client {
    stream: TcpStream,
}

impl Client {

    pub fn new(serverIP: String) -> Result<Self, Box<dyn Error>> {
        let portNumber = 34254;
        let serverAddr = format!("{}:{}", serverIP, portNumber);
        let mut st = TcpStream::connect(serverAddr)?;
        let peerAddr = st.peer_addr()?;
        let peerAddrString = peerAddr.to_string();
        println!("00. => Connected to Server: {}", peerAddrString);
        let localAddr = st.local_addr()?;
        let localAddrString = localAddr.to_string();
        println!("00. => From local: {}", localAddrString);
        Ok(Self{stream: st})
    }

    fn getDate(&mut self) {
        let theDateCommand = String::from("GetDate");
        println!("01. -? Sending Command {} to the server..", theDateCommand);
        // Send the Date Command
        //self.send(theDateCommand);

        // Receive the Date and time
        
    }

}