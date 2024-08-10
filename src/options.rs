use clap::{Parser, ValueEnum};
use std::net::{IpAddr, SocketAddr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Model {
    AddAgent,
    GetAgents,
    UpdateAgent,
    RemoveAgent,
    Emulate,
    GetTask,
    FreshDB,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TaskMode {
    Sequence,
    Aggregation,
    Max,
    Min,
}

impl std::fmt::Display for TaskMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskMode::Sequence => write!(f, "sequence"),
            TaskMode::Aggregation => write!(f, "aggregation"),
            TaskMode::Max => write!(f, "max"),
            TaskMode::Min => write!(f, "min"),
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
    #[arg(short, long, default_value = "1")]
    pub shots: usize,

    /// The number of qubits
    #[arg(short, long, default_value = "1")]
    pub qubits: usize,

    /// The depth of the circuit
    #[arg(short, long, default_value = "1")]
    pub depth: usize,

    /// The mode of the task
    #[arg(long, default_value = None)]
    pub task_mode: Option<TaskMode>,

    /// The agent uuid
    #[arg(long, default_value = None)]
    pub agent_id: Option<String>,

    /// The agent ip when initializing the database
    #[arg(long, default_value = None)]
    pub agent_ip: Option<IpAddr>,

    /// The agent hostname when initializing the database
    #[arg(long, default_value = None)]
    pub agent_hostname: Option<String>,

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
