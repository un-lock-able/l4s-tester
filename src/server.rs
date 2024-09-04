use std::{io::Read, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};

pub struct TestServer {
    pub addr: SocketAddrV4,
}

impl TestServer {
    pub fn new(port: u16) -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)
        }
    }

    pub fn start_listen(&self) {
        let listener = TcpListener::bind(self.addr).expect("Server bind error");
        println!("Looping for incoming stream");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    Self::handle_incoming_stream(stream);
                }
                Err(e) => {
                    println!("Receive stream err: {}", e);
                }
            }
        }
    }

    fn handle_incoming_stream(mut stream: TcpStream) {
        // We simply want to consume the content
        let mut buf = Vec::<u8>::new();
        match stream.read_to_end(&mut buf) {
            Ok(_) => {
                println!("Receive data length: {}, content: {}", buf.len(), &buf.get(0).unwrap_or(&0));
                return;
            },
            Err(e) => println!("Receive error: {}", e),
        }
    }
}