use std::net::{TcpStream};
use std::error::Error;
use std::io::{Read, Write};

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
        st.write(b"Hello Server!")?;
        Ok(Self{stream: st})
    }

}