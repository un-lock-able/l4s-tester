use std::net::Ipv4Addr;

use clap::{Parser, Subcommand};
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(name = "l4s-tester", version, propagate_version = true)]
#[command(about = "A tester for testing AQM")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long = "threads", default_value_t = 4)]
    pub threads_num: usize,
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    Server {
        #[arg(short, value_name = "Listen Port")]
        port: u16,
    },
    Client {
        #[arg(long, value_parser = humantime::parse_duration)]
        test_time: Option<Duration>,
        #[arg(short, value_name = "Server Address")]
        addr: Ipv4Addr,
        #[arg(short, value_name = "Server Port")]
        port: u16,
        #[arg(long)]
        arrive_per_sec: u16,
        #[arg(long, default_value_t = 0.9)]
        alpha: f64,
    }
}