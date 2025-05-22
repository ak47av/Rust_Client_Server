#![allow(non_snake_case)]
use std::thread;
use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};
use std::error::Error;

use crate::Communication::Communication;
use crate::DateTimeService;

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
            println!("Received command from client {}: {}", self.connection_number, decoded);

            if decoded == "GetDate" {
                println!("Sending the date and time to the client!");
                self.getDate();
            }
        }

        true
    }

    fn getDate(&mut self) {
        let currentDateTimeText = DateTimeService::getDateAndTime();
        let mut buffer = [0u8; 100];
        let _length = ThreadedConnectionHandler::encode(currentDateTimeText, &mut buffer);
        let _ = self.send(&mut buffer);
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