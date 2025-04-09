use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;

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
                let mut buf = [0; 50];
                self.stream.read(&mut buf);
                match str::from_utf8(&buf) {
                    Ok(x) => println!("Client said: {}", x),
                    Err(err) => println!("ERROR")
                }
                while self.readCommand() {}
            });
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