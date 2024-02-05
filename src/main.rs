use clap::Parser;
extern crate reqwest;
mod options;
use serde_json::Value;
use std::fs;

fn init_qthread(cli: &options::Options) -> String {
    serde_json::to_string_pretty(
        &reqwest::blocking::get(format!("http://{}/init", cli.address))
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn emulate(cli: &options::Options) -> String {
    let content = fs::read_to_string(&cli.file).expect("Something went wrong reading the file");
    println!("With text:\n{}", content);
    let body = [
        ("qasm", content),
        ("shots", cli.shots.to_string()),
        ("format", cli.outputformat.to_string()),
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

fn main() {
    let cli = options::Options::parse();

    let output = if cli.init_db { init_qthread(&cli) } else { emulate(&cli) };

    println!("{}", output);
}
