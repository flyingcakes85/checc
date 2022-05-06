use serde::{Deserialize, Serialize};

// config struct
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub build: String,
    pub run: String,
    pub input: String,
    pub output: String,
}

impl Config {
    // TODO: maybe not needed?
    pub fn _new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }
        let build = args[1].clone();
        let run = args[2].clone();
        let input = args[3].clone();
        let output = args[4].clone();
        Ok(Config {
            build,
            run,
            input,
            output,
        })
    }
}
