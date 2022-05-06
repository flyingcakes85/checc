use crate::config::Config;
use std::{fs, process};
mod config;

fn main() {
    // read config file and store in config struct
    let config: Result<Config, serde_yaml::Error> =
        serde_yaml::from_str(fs::read_to_string("./.checc.yml").unwrap().as_str());

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            println!("Problem parsing config file:\n{}", e);
            process::exit(1);
        }
    };

    println!("Build: {}", config.build);
    println!("Run: {}", config.run);
    println!("Input: {}", config.input);
    println!("Output: {}", config.output);
}
