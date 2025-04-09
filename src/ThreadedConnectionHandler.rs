use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;
use std::error::Error;

pub struct ThreadedConnectionHandler {
    stream: TcpStream,
    connection_number: u8
}

impl ThreadedConnectionHandler {

    pub fn new (stream: TcpStream, connection_number: u8) {
        let mut instance = ThreadedConnectionHandler{stream, connection_number};
        thread::spawn(move || {
            while instance.readCommand() {}
        });
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

    fn readCommand(&mut self) -> bool {
        // let mut buf = [0; 50];
        // self.stream.read(&mut buf);
        // match str::from_utf8(&mut buf) {
        //     Ok(x) => {
        //         let trimmed = x.trim_matches(char::from(0)).trim();
        //         if !trimmed.is_empty() {
        //             println!("Received a String object from the client ({}).", trimmed);
        //         }
        //     },
        //     Err(_) => { self.closeStream(); return false;}
        // }
        self.receive();
        true
    }

    fn closeStream(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }

}