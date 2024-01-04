pub mod tcp_socket {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    // Client
    pub struct Client {
        pub addr: String,
        pub cli: TcpStream
    }

    pub fn create_client(addr: &str) -> Client {
        return Client {addr: addr.to_string(),cli: TcpStream::connect(addr).unwrap()};
    }

    pub fn close_client(mut sock: &TcpStream) {
        sock.flush().unwrap();
    }

    pub fn test() {
        println!("Testing Sock");
    }

    // Server
    pub struct Server {
        pub addr: String,
        pub ser: TcpListener
    }

    pub fn create_server(addr: &str) -> Server {
        return Server {addr: addr.to_string(),ser: TcpListener::bind(addr.to_string()).unwrap()};
    }

    // server and client funcs
    pub fn send(mut sock: &TcpStream, msg: &str) {
        sock.write_all(msg.as_bytes()).unwrap();
    }

    pub fn recv(mut sock: &TcpStream, buf: usize) -> String {
        let mut buffer = vec![0; buf];
        let size = sock.read(&mut buffer).unwrap();
        return String::from_utf8_lossy(&buffer[..size]).to_string();
    }
}

pub mod udp_socket {
    use std::net::UdpSocket;

    pub struct Socket {
        pub addr: String,
        pub sock: UdpSocket
    }

    pub fn create_socket(addr: &str) -> Socket {
        return Socket {addr: addr.to_string(), sock: UdpSocket::bind("127.0.0.1").expect("err")};
    }

    impl Socket {
        pub fn send_to(&self, msg: &str, target_addr: &String) {
            self.sock.send_to(msg.as_bytes(), target_addr).expect("Failed to send message");
        }

        pub fn recv_from(&self, buf: usize) -> (String, String) {
            let mut buffer = vec![0; buf];
            let (size, src_addr) = self.sock.recv_from(&mut buffer).expect("Failed to receive response");
            let response = String::from_utf8_lossy(&buffer[..size]).to_string();

            return (response,src_addr.to_string());
        }
    }
}