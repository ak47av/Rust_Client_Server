use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;

pub struct ThreadedConnectionHandler {
    stream: TcpStream,
}

impl ThreadedConnectionHandler {

    pub fn run(&mut self) {
        thread::scope(|s| {
            s.spawn(|| {
                let mut buf = [0; 50];
                self.stream.read(&mut buf);
                match str::from_utf8(&buf) {
                    Ok(x) => println!("Client said: {}", x),
                    Err(err) => println!("ERROR")
                }
                self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            });
        });
    }

    pub fn new (stream: TcpStream) -> Self {
        Self { stream }
    }
}