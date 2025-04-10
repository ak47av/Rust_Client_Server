use std::net::{TcpListener, TcpStream};
use tcpexample::ThreadedConnectionHandler::ThreadedConnectionHandler;

/*
Handle incoming TCP Streams as separate clients
*/
fn handle_client(stream: TcpStream, conn_no: &mut u8) {
    println!("A Client connected! Issued no: {}", conn_no);
    ThreadedConnectionHandler::new(stream, *conn_no);
    *conn_no += 1;
    //tch.run();
}

fn main() -> std::io::Result<()> {
    // Bind the server to localhost port 34254
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    let mut connection_number = 1;
    println!("Listening for a connection..");

    // Process incoming connections
    for stream in listener.incoming() {
        handle_client(stream?, &mut connection_number);
    }

    Ok(())
}