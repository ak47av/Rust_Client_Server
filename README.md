This is a Threaded Server application with a Client, written completely in Rust.

The /bin folder contains the tcpsender(Client) and tcplistener(Server) applications which utilise the other structs:
- Client
- DateTimeService
- ThreadedConnectionHandler

The Communication trait implements the encoding/decoding functionality of this application so that instances of any structs can be encoded and decoded.

To build the application, run
```
foo@bar:~$ cargo build
```

To run the applications, first run the tcplistener application:
```
foo@bar:~$ cargo run --bin tcplistener
```
and then run the tcpsender application:
```
foo@bar:~$ cargo run --bin tcpsender
```

Functionality is still being added to the applications.
