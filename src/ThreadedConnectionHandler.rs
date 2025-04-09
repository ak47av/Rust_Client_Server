use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;
use std::error::Error;

pub struct ThreadedConnectionHandler {
    stream: TcpStream,
}

impl ThreadedConnectionHandler {

    pub fn new (stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn run(&mut self) {
        thread::scope(|s| {
            s.spawn(|| {
                while self.readCommand() {}
            });
        });
    }
    
    pub fn receive(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>>{
        let mut buffer = [0u8; 50];
        let bytesRead = self.stream.read(&mut buffer)?;

        let deserialized: u8 = bincode::decode_from_slice(&mut buffer, bincode::config::standard())?.0;
        buf[0] = deserialized;
        Ok(bytesRead)
    }

    fn readCommand(&mut self) -> bool {
        let mut buf = [0; 50];
        let bytesReceived = match self.receive(&mut buf) {
            Ok(bytes) => bytes,
            Err(e) => 0
        };
        if bytesReceived > 0 {
            println!("Received a String object from the client ({}).", buf[0]);
        }
        true
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



    fn closeStream(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }

}