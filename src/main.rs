use std::str::FromStr;

use clap::Parser;
use client::TestClient;
use config::Cli;
use server::TestServer;
use humantime::Duration;

mod config;
mod server;
mod client;

fn main() {
    let args = Cli::parse();
    match args.command {
        config::Commands::Server{port} => {
            let server = TestServer::new(port, args.threads_num);
            server.start_listen();
        }
        config::Commands::Client {test_time, addr, port , arrive_per_sec, alpha} => {
            let mut client = TestClient::new(test_time.unwrap_or(Duration::from_str("250s").expect("Cannot parse 250s as duration").into()),addr, port, arrive_per_sec, alpha, args.threads_num);
            client.start_sending();
        }
    }
}
