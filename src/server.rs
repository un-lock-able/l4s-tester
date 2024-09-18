use std::{io::Read, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};

pub struct TestServer {
    addr: SocketAddrV4,
    thread_num: usize,
}

impl TestServer {
    pub fn new(port: u16, thread_num: usize) -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port),
            thread_num
        }
    }

    pub fn start_listen(&self) {
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(self.thread_num).build().expect("Build threadpool failed");
        let listener = TcpListener::bind(self.addr).expect("Server bind error");
        println!("Looping for incoming stream");
        thread_pool.scope(|s| {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        // thread::spawn(|| {Self::handle_incoming_stream(stream);});
                        s.spawn(move |_| {Self::handle_incoming_stream(stream);})
                    }
                    Err(e) => {
                        println!("Receive stream err: {}", e);
                    }
                }
            }
        })
    }

    fn handle_incoming_stream(mut stream: TcpStream) {
        // We simply want to consume the content
        let mut buf = Vec::<u8>::new();
        match stream.read_to_end(&mut buf) {
            Ok(_) => {
                println!("Receive data length: {}, flow count: {}, content: {}", buf.len(), u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]), &buf.get(4).unwrap_or(&0));
            },
            Err(e) => println!("Receive error: {}", e),
        }
    }
}