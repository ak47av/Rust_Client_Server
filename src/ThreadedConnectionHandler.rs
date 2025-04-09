use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;

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

    fn readCommand(&mut self) -> bool {
        let mut buf = [0; 50];
        self.stream.read(&mut buf);
        match str::from_utf8(&mut buf) {
            Ok(x) => {
                let trimmed = x.trim_matches(char::from(0)).trim();
                if !trimmed.is_empty() {
                    println!("Received a String object from the client ({}).", trimmed);
                }
            },
            Err(_) => { self.closeStream(); return false;}
        }
        true
    }

    fn closeStream(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }

}