use clap::{Parser, ValueEnum};
use std::net::{IpAddr, SocketAddr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Model {
    AddAgent,
    GetAgents,
    UpdateAgent,
    Emulate,
    GetTask,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AgentStatus {
    Down,
    Running,
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Down => write!(f, "down"),
            AgentStatus::Running => write!(f, "running"),
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

    /// The number of shots
    #[arg(short, long, default_value = "0")]
    pub shots: usize,

    /// The agent uuid
    #[arg(long, default_value = None)]
    pub agent_id: Option<String>,

    /// The agent ip when initializing the database
    #[arg(long, default_value = None)]
    pub agent_ip: Option<IpAddr>,

    /// The agent port when initializing the database
    #[arg(long, default_value = None)]
    pub agent_port: Option<u16>,

    /// The agent qubit
    #[arg(long, default_value = None)]
    pub agent_qubit_count: Option<usize>,

    /// The agent circuit depth
    #[arg(long, default_value = None)]
    pub agent_circuit_depth: Option<usize>,

    /// The agent status
    #[arg(long, default_value = None)]
    pub agent_status: Option<AgentStatus>,

    /// The id of the job
    #[arg(short, long, default_value = "")]
    pub task_id: String,

    /// Whether to test the server, do not need to specify the file
    #[arg(short, long, default_value = "emulate")]
    pub model: Model,
}
