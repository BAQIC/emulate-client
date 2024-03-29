use clap::{Parser, ValueEnum};
use std::net::{IpAddr, SocketAddr};

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
    CUDAQ,
    InitQthread,
    Emulate,
    GetTask,
    Qpp,
    #[clap(name = "qasmsim")]
    QASMSim,
    Agent,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SimulatorType {
    DM,
    SV,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AgentType {
    QppSV,
    QppDM,
    #[clap(name = "qasmsim")]
    QASMSim,
    CUDAQ,
}

impl std::fmt::Display for SimulatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulatorType::DM => write!(f, "dm"),
            SimulatorType::SV => write!(f, "sv"),
        }
    }
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::QppSV => write!(f, "qpp-sv"),
            AgentType::QppDM => write!(f, "qpp-dm"),
            AgentType::QASMSim => write!(f, "qasmsim"),
            AgentType::CUDAQ => write!(f, "cudaq"),
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

    /// The number of shots
    #[arg(short, long, default_value = "0")]
    pub shots: usize,

    /// The agent ip when initializing the database
    #[arg(long, default_value = "127.0.0.1")]
    pub agent_ip: IpAddr,

    /// The agent port when initializing the database
    #[arg(long, default_value = "3000")]
    pub agent_port: u16,

    /// The id of the job
    #[arg(short, long, default_value = None)]
    pub task_id: Option<String>,

    /// Whether to test the server, do not need to specify the file
    #[arg(short, long, default_value = "emulate")]
    pub model: Model,

    /// The simulator type, dm represents density matrix, sv represents state vector
    #[arg(long, default_value = "dm")]
    pub simulator: SimulatorType,

    /// The agent type
    #[arg(long, default_value = "qasmsim")]
    pub agent: AgentType,
}
