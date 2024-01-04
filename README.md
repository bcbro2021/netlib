# netlib
A short networking library made with and for rust.

## Installation
run this inside your project folder- <br />
```sh
git clone https://github.com/bcbro2021/netlib.git
```
Inside Cargo.toml - <br />
```toml
[dependencies]
netlib={path="./netlib"}
```
## Getting Started
### Client
```rust
use netlib::socket;

fn main() {
    // creating the client socket
    let client: socket::Client = socket::create_client("127.0.0.1:1234");

    loop {
        // recieving the server message
        let msg = socket::recv(&client.cli,2048);
        println!("{}", msg);

        // sending a message to the server
        socket::send(&client.cli,"Hi Server....");
    }
    
}
```
### Server
```rust
use netlib::socket;
use std::{thread, net::TcpStream};

// handle multiple clients at once
fn handle_client(client: TcpStream) {
    loop {
        // send and recieve data
        socket::send(&client, "Hi client...");
        let msg = socket::recv(&client, 2048);
        println!("{}", msg);
    }
}

fn main() {
    // server socket
    let server: socket::Server = socket::create_server("127.0.0.1:1234");

    // check for client connections and create a new thread
    for cl in server.ser.incoming() {
        let client = cl.unwrap();

        thread::spawn(move || {handle_client(client)});
    }
}
```
