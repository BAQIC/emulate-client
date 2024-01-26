use clap::Parser;
extern crate reqwest;
mod options;
use serde_json::Value;

fn test(cli: &options::Options) -> String {
    serde_json::to_string_pretty(
        &reqwest::blocking::get(format!("http://{}", cli.address))
            .unwrap()
            .json::<Value>()
            .unwrap(),
    )
    .unwrap()
}

fn emulate() -> String {
    String::from("emulate")
}

fn main() {
    let cli = options::Options::parse();

    let output = if cli.test { test(&cli) } else { emulate() };

    println!("{}", output);
}
