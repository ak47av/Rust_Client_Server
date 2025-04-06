use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream, conn_no: &mut u8) {
    println!("A Client connected! Issued no: {}", conn_no);
    *conn_no += 1;
    thread::spawn(move || {
        let mut buf = [0; 10];
        stream.read(&mut buf);
        match str::from_utf8(&buf) {
            Ok(x) => println!("Client said: {}", x),
            Err(err) => println!("ERROR")
        }
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    });
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    let mut connection_number = 1;

    for stream in listener.incoming() {
        handle_client(stream?, &mut connection_number);
    }

    Ok(())
}