//! # Quantum Emulator Client
//! This is a client for the Quantum Emulator Server. It allows you to add, get,
//! update, and remove agents, submit tasks, and get the results of tasks.

use clap::Parser;
extern crate reqwest;
mod options;
use serde_json::Value;
use std::{fs, vec};

fn add_agent(cli: &options::Options) -> String {
    let mut params: Vec<(String, String)> = vec![
        ("port".to_string(), cli.agent_port.unwrap().to_string()),
        (
            "qubit_count".to_string(),
            cli.agent_qubit_count.unwrap().to_string(),
        ),
        (
            "circuit_depth".to_string(),
            cli.agent_circuit_depth.unwrap().to_string(),
        ),
    ];

    if cli.agent_ip.is_some() {
        params.push(("ip".to_string(), cli.agent_ip.unwrap().to_string()));
    } else {
        params.push(("ip".to_string(), "".to_string()));
        params.push((
            "hostname".to_string(),
            cli.agent_hostname.clone().unwrap().to_string(),
        ));
    }

    serde_json::to_string_pretty(
        &reqwest::blocking::Client::new()
            .post(format!("http://{}/add_agent", cli.address))
            .form(&params)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn get_agents(cli: &options::Options) -> String {
    match cli.agent_port {
        Some(port) => {
            if cli.agent_ip.is_none() {
                let url = reqwest::Url::parse_with_params(
                    &format!("http://{}/get_agents", cli.address),
                    [
                        ("ip", "".to_string()),
                        ("hostname", cli.agent_hostname.clone().unwrap()),
                        ("port", port.to_string()),
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
            } else {
                let url = reqwest::Url::parse_with_params(
                    &format!("http://{}/get_agents", cli.address),
                    [
                        ("ip", cli.agent_ip.unwrap().to_string()),
                        ("port", port.to_string()),
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
        }
        None => {
            if cli.agent_ip.is_none() {
                let url = reqwest::Url::parse_with_params(
                    &format!("http://{}/get_agents", cli.address),
                    [
                        ("ip", "".to_string()),
                        ("hostname", cli.agent_hostname.clone().unwrap()),
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
            } else {
                let url = reqwest::Url::parse_with_params(
                    &format!("http://{}/get_agents", cli.address),
                    [("ip", cli.agent_ip.unwrap().to_string())],
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
}

fn update_agent(cli: &options::Options) -> String {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("id".to_string(), cli.agent_id.clone().unwrap()));

    if let Some(ip) = cli.agent_ip {
        params.push(("ip".to_string(), ip.to_string()));
    }
    if let Some(port) = cli.agent_port {
        params.push(("port".to_string(), port.to_string()));
    }
    if let Some(qubit_count) = cli.agent_qubit_count {
        params.push(("qubit_count".to_string(), qubit_count.to_string()));
    }
    if let Some(circuit_depth) = cli.agent_circuit_depth {
        params.push(("circuit_depth".to_string(), circuit_depth.to_string()));
    }
    if let Some(status) = cli.agent_status {
        params.push(("status".to_string(), status.to_string()));
    }

    serde_json::to_string_pretty(
        &reqwest::blocking::Client::new()
            .post(format!("http://{}/update_agent", cli.address))
            .form(&params)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn remove_agent(cli: &options::Options) -> String {
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/remove_agent", cli.address),
        [("id", &cli.agent_id.clone().unwrap())],
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

fn qasmsim_agent(cli: &options::Options) -> String {
    let content = fs::read_to_string(&cli.file).expect("Something went wrong reading the file");
    let mut body = vec![
        ("task_id", "test".to_string()),
        ("qasm", content),
        ("shots", cli.shots.to_string()),
    ];

    if cli.task_mode.is_some() {
        body.push(("mode", cli.task_mode.unwrap().to_string()));
    }

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

fn update_qasmsim_agent(cli: &options::Options) -> String {
    let body = vec![
        ("qbits", cli.qubits.to_string()),
        ("capacity", cli.capacity.to_string()),
    ];

    serde_json::to_string_pretty(
        &reqwest::blocking::Client::new()
            .post(format!("http://{}/update", cli.address))
            .form(&body)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn get_qasmsim_agent(cli: &options::Options) -> String {
    let url = reqwest::Url::parse_with_params(
        &format!("http://{}/get_measure", cli.address),
        [("pos", &cli.position.to_string())],
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
    let mut body = vec![
        ("code", content),
        ("shots", cli.shots.to_string()),
        ("depth", cli.depth.to_string()),
        ("qubits", cli.qubits.to_string()),
    ];
    if cli.task_mode.is_some() {
        body.push(("mode", cli.task_mode.unwrap().to_string()));
    }
    if cli.vars.is_some() {
        let vars = fs::read_to_string(cli.vars.clone().unwrap())
            .expect("Something went wrong reading the file");
        body.push(("vars", vars));
    }

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

fn fresh_db(cli: &options::Options) -> String {
    serde_json::to_string_pretty(
        &reqwest::blocking::Client::new()
            .post(format!("http://{}/fresh_db", cli.address))
            .send()
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
        options::Model::RemoveAgent => remove_agent(&cli),
        options::Model::QasmSimAgent => qasmsim_agent(&cli),
        options::Model::UpdateQasmSimAgent => update_qasmsim_agent(&cli),
        options::Model::GetQasmSimAgent => get_qasmsim_agent(&cli),
        options::Model::Emulate => emulate(&cli),
        options::Model::GetTask => get_task(&cli),
        options::Model::FreshDB => fresh_db(&cli),
    };

    println!("{}", output);
}
