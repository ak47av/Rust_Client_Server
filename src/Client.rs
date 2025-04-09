use std::net::{TcpStream};
use std::error::Error;
use std::io::{Read, Write};
use crate::DateTimeService;
use std::str;

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

    pub fn send(&mut self, object: &impl bincode::Encode) -> Result<(), Box<dyn Error>>{
        println!("02. -> Sending an object...");
        let mut buffer = [0u8; 50];
        let mut bytes = bincode::encode_into_slice(
            object,
            &mut buffer,
            bincode::config::standard()
        )?;
        self.stream.write(&mut buffer[..bytes])?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0u8; 100];
        let bytes_received = self.stream.read(&mut buffer);

        let deserialized: [u8; 100] = bincode::decode_from_slice(&mut buffer, bincode::config::standard())?.0;
        match str::from_utf8(&deserialized) {
            Ok(x) => println!("Received Object: {}", x),
            Err(err) => println!("Error with received object")
        }
        Ok(())
    }

}