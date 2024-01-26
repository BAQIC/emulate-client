use clap::{Parser, ValueEnum};
use std::net::SocketAddr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Json,
    Tableau,
}

#[derive(Parser, Debug)]
#[command(name = "emulate-client")]
#[command(author = "Lucky <lucky@lucky9.cyou>")]
#[command(version = "1.0")]
#[command(about = "OpenQASM 2.0 emulate client", long_about = None)]
pub struct Options {
    /// The path to the file to read
    #[arg(short, long, default_value = "example.qasm")]
    pub file: String,

    #[arg(short, long, default_value = "127.0.0.1:3000")]
    /// The server address
    pub address: SocketAddr,

    /// The output format
    #[arg(short, long, value_enum, default_value = "json")]
    pub outputformat: OutputFormat,

    /// The number of shots
    #[arg(short, long, default_value = "1")]
    pub shots: usize,

    /// Whether to test the server, do not need to specify the file
    #[arg(short, long)]
    pub test: bool,
}
