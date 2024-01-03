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
    let client: socket::Client = socket::create_client("127.0.0.1:1234");

    loop {
        let msg = socket::recv(&client.cli,2048);
        println!("{}", msg);
        socket::send(&client.cli,"Hi server....");
    }
    
}
```
### Server
```rust
use netlib::socket;
use std::{thread, net::TcpStream};

fn handle_client(client: TcpStream) {
    loop {
        socket::send(&client, "Hi Client...");
        let msg = socket::recv(&client, 2048);
        println!("{}", msg);
    }
}

fn main() {
    let server: socket::Server = socket::create_server("127.0.0.1:1234");

    for cl in server.ser.incoming() {
        let client = cl.unwrap();

        thread::spawn(move || {handle_client(client)});
    }
}
```
