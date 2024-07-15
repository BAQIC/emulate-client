use clap::Parser;
extern crate reqwest;
mod options;
use serde_json::Value;
use std::fs;

fn add_agent(cli: &options::Options) -> String {
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/add_agent", cli.address),
        [
            ("ip", cli.agent_ip.to_string()),
            ("port", cli.agent_port.unwrap().to_string()),
            ("qubit_count", cli.agent_qubit_count.to_string()),
            ("circuit_depth", cli.agent_circuit_depth.to_string()),
        ],
    )
    .unwrap();
    serde_json::to_string_pretty(
        &reqwest::blocking::get(url)
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn get_agents(cli: &options::Options) -> String {
    match cli.agent_port {
        Some(port) => {
            let url = reqwest::Url::parse_with_params(
                &format!("http://{}/get_agents", cli.address),
                [("ip", cli.agent_ip.to_string()), ("port", port.to_string())],
            )
            .unwrap();
            serde_json::to_string_pretty(
                &reqwest::blocking::get(url)
                    .unwrap()
                    .json::<Value>()
                    .unwrap(),
            )
            .unwrap()
        }
        None => {
            let url = reqwest::Url::parse_with_params(
                &format!("http://{}/get_agents", cli.address),
                [("ip", cli.agent_ip.to_string())],
            )
            .unwrap();
            serde_json::to_string_pretty(
                &reqwest::blocking::get(url)
                    .unwrap()
                    .json::<Value>()
                    .unwrap(),
            )
            .unwrap()
        }
    }
}

fn update_agent(cli: &options::Options) -> String {
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/update_agent", cli.address),
        [
            ("id", cli.agent_id.to_string()),
            ("ip", cli.agent_ip.to_string()),
            ("port", cli.agent_port.unwrap().to_string()),
            ("qubit_count", cli.agent_qubit_count.to_string()),
            ("circuit_depth", cli.agent_circuit_depth.to_string()),
        ],
    )
    .unwrap();
    serde_json::to_string_pretty(
        &reqwest::blocking::get(url)
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn update_agent_status(cli: &options::Options) -> String {
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/update_agent_status", cli.address),
        [
            ("id", cli.agent_id.to_string()),
            ("status", cli.agent_status.to_string()),
        ],
    )
    .unwrap();
    serde_json::to_string_pretty(
        &reqwest::blocking::get(url)
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn emulate(cli: &options::Options) -> String {
    let content = fs::read_to_string(&cli.file).expect("Something went wrong reading the file");
    let body = [
        ("code", content),
        ("shots", cli.shots.to_string()),
        ("depth", 10.to_string()),
        ("qubits", 10.to_string()),
    ];
    serde_json::to_string_pretty(
        &reqwest::blocking::Client::new()
            .post(format!("http://{}/submit", cli.address))
            .form(&body)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn get_task(cli: &options::Options) -> String {
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/get_task", cli.address),
        [("task_id", &cli.task_id)],
    )
    .unwrap();
    serde_json::to_string_pretty(
        &reqwest::blocking::get(url)
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn main() {
    let cli = options::Options::parse();

    let output = match cli.model {
        options::Model::AddAgent => add_agent(&cli),
        options::Model::GetAgents => get_agents(&cli),
        options::Model::UpdateAgent => update_agent(&cli),
        options::Model::UpdateAgentStatus => update_agent_status(&cli),
        options::Model::Emulate => emulate(&cli),
        options::Model::GetTask => get_task(&cli),
    };

    println!("{}", output);
}
