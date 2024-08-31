use std::{io::Write, net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpStream}, thread::{self, sleep, JoinHandle}, time::{Duration, Instant}};

use rand::{rngs::StdRng, RngCore, SeedableRng};
use rand_distr::{Distribution, Exp, Pareto};

pub struct TestClient {
    test_time: Duration,
    addr: SocketAddrV4,
    pareto_dist: Pareto<f64>,
    exp_dist: Exp<f64>,
    start_time: Instant,
    rng: StdRng,
    threads: Vec<JoinHandle<()>>
}

impl TestClient {
    pub fn new(test_time: Duration, ip: Ipv4Addr, port: u16, arrive_per_sec: u16, alpha: f64) -> Self {
        Self {
            test_time,
            addr: SocketAddrV4::new(ip, port),
            pareto_dist: Pareto::new(alpha, 1.0).expect("Create pareto distribution with alpha = {} failed"),
            exp_dist: Exp::new(arrive_per_sec.into()).expect("Create Exponential distribution with lambda = {} failed"),
            start_time: Instant::now(),
            rng: StdRng::from_entropy(),
            threads: Vec::new(),
        }
    }

    pub fn start_sending(&mut self) {
        self.start_time = Instant::now();
        let mut next_send_time = self.start_time + Duration::from_micros((self.exp_dist.sample(&mut self.rng) * 1000000.0) as u64);
        while self.start_time.elapsed() < self.test_time {
            println!("Elapsed: {}", self.start_time.elapsed().as_secs());
            let now = Instant::now();
            let send_size = self.get_send_size();
            let content = self.rng.next_u32() as u8;
            println!("Data length: {}, content: {}", send_size * 1024, content);
            
            let addr = self.addr;
            let buf: Vec<u8> = vec![content; send_size as usize * 1024];
            sleep(next_send_time - now);

            self.threads.push(thread::spawn(move || {
                let mut stream = TcpStream::connect(addr).expect("Couldn't connect to the server...");
    
                // stream.set_nonblocking(true).expect("set_nonblocking call failed");
    
                stream.write_all(&buf).expect("Stream write failed");
                stream.shutdown(Shutdown::Both).expect("Stream shutdown failed");
            }));

            next_send_time += Duration::from_micros((self.exp_dist.sample(&mut self.rng) * 1000000.0) as u64);
        }

        while let Some(handle) = self.threads.pop() {
            handle.join().expect("Join thread failed");
        }
    }

    /// Return a packet size in the unit of KB
    fn get_send_size(&mut self) -> u32 {
        loop {
            let size = self.pareto_dist.sample(&mut self.rng) as u32;
            if size <= 1024 && size >= 1 {
                return size;
            }
        }
    }
}