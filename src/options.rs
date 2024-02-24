use clap::{Parser, ValueEnum};
use std::net::SocketAddr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Json,
    Tabular,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Tabular => write!(f, "tabular"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Model {
    InitQthread,
    Emulate,
    GetTask,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Model::InitQthread => write!(f, "init-qthread"),
            Model::Emulate => write!(f, "emulate"),
            Model::GetTask => write!(f, "get-task"),
        }
    }
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

    /// The number of shots1
    #[arg(short, long, default_value = "0")]
    pub shots: usize,

    /// The number of agents when initializing the database
    #[arg(long, default_value = "1")]
    pub agent_num: usize,

    /// The id of the job
    #[arg(short, long, default_value = None)]
    pub task_id: Option<String>,

    /// Whether to test the server, do not need to specify the file
    #[arg(short, long, default_value = "emulate")]
    pub model: Model,
}
