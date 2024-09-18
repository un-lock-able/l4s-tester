use std::{io::Write, net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpStream}, thread::sleep, time::{Duration, Instant}};

use rand::{rngs::StdRng, RngCore, SeedableRng};
use rand_distr::{Distribution, Exp, Pareto};

pub struct TestClient {
    test_time: Duration,
    addr: SocketAddrV4,
    pareto_dist: Pareto<f64>,
    exp_dist: Exp<f64>,
    rng: StdRng,
    thread_num: usize,
}

impl TestClient {
    pub fn new(test_time: Duration, ip: Ipv4Addr, port: u16, arrive_per_sec: u16, alpha: f64, thread_num: usize) -> Self {
        Self {
            test_time,
            addr: SocketAddrV4::new(ip, port),
            pareto_dist: Pareto::new(alpha, 1.0).expect("Create pareto distribution with alpha = {} failed"),
            exp_dist: Exp::new(arrive_per_sec.into()).expect("Create Exponential distribution with lambda = {} failed"),
            rng: StdRng::from_entropy(),
            thread_num,
        }
    }
    
    pub fn start_sending(&mut self) {
        let start_time = Instant::now();
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(self.thread_num).build().expect("Build threadpool failed");
        thread_pool.scope(|s| {
            let mut send_count: u32 = 0;
            let mut next_send_time = start_time + Duration::from_micros((self.exp_dist.sample(&mut self.rng) * 1000000.0) as u64);
            while start_time.elapsed() < self.test_time {
                println!("Elapsed: {}", start_time.elapsed().as_secs());
                let now = Instant::now();
                let send_size = self.get_send_size();
                let content = self.rng.next_u32() as u8;
                println!("Send data length: {}, flow_count: {}, content: {}", send_size * 1024, send_count, content);
                
                let addr = self.addr;
                let mut buf: Vec<u8> = vec![content; send_size as usize * 1024];
                buf[0..4].copy_from_slice(&send_count.to_be_bytes());

                sleep(next_send_time - now);

                s.spawn(move |_| {
                    let mut stream = TcpStream::connect(addr).expect("Couldn't connect to the server...");
        
                    // stream.set_nonblocking(true).expect("set_nonblocking call failed");
                    stream.set_nodelay(true).expect("Set nodelay failed");
                    stream.write_all(&buf).expect("Stream write failed");
                    stream.shutdown(Shutdown::Both).expect("Stream shutdown failed"); 
                    drop(stream);
                });

                next_send_time += Duration::from_micros((self.exp_dist.sample(&mut self.rng) * 1000000.0) as u64);
                send_count += 1;
            }
        });
    }

    /// Return a packet size in the unit of KB
    fn get_send_size(&mut self) -> u32 {
        loop {
            let size = self.pareto_dist.sample(&mut self.rng) as u32;
            if (1..=1024).contains(&size) {
                return size;
            }
        }
    }
}