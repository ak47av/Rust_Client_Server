use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;
use std::error::Error;

use crate::Communication::Communication;

pub struct ThreadedConnectionHandler {
    stream: TcpStream,
    connection_number: u8
}

impl ThreadedConnectionHandler {

    // Initialize a threaded for every connection from a client
    pub fn new (stream: TcpStream, connection_number: u8) {
        // declare are instance of the struct to get a handle to the struct
        let mut instance = ThreadedConnectionHandler{stream, connection_number};

        //spawn a thread which listens for commands
        thread::spawn(move || {
            while instance.readCommand() {}
        });
    }

    // Reads the command from the TCP stream
    fn readCommand(&mut self) -> bool {
        let mut buffer = [0u8; 100];
        let bytes = self.receive(&mut buffer).unwrap();

        // If something was received from the stream, decode it and print the command
        if bytes > 0 {
            let decoded: String = ThreadedConnectionHandler::decode(bytes, &mut buffer).unwrap();
            println!("Got command: {}", decoded);
        }
        true
    }

    // Closes the TCP Stream
    fn closeStream(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }

}

// Implements the Communication Trait
impl Communication for ThreadedConnectionHandler {
    fn send(&mut self, buf: &mut[u8]) -> Result<(), Box<dyn Error>>{
        self.stream.write(buf)?;
        Ok(())
    }

    fn receive(&mut self, buf: &mut[u8]) -> Result<usize, Box<dyn Error>> {
        let bytes_received = self.stream.read(buf)?;
        Ok(bytes_received)
    }
}