use crate::config::Config;
use prettydiff::diff_lines;
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

    // println!("Build: {}", config.build);
    // println!("Run: {}", config.run);
    // println!("Input: {}", config.input);
    // println!("Output: {}", config.output);

    println!("Starting build...");
    println!("Running: {}", config.build);
    // use shell-words to store build command in a vector
    let build_cmd = shell_words::split(&config.build).unwrap();
    let build_output = process::Command::new(build_cmd[0].clone())
        .args(&build_cmd[1..])
        .output();

    match build_output {
        Ok(build_output) => {
            println!(
                "Build output:\n{}",
                String::from_utf8_lossy(&build_output.stdout)
            );
            println!(
                "Build error:\n{}",
                String::from_utf8_lossy(&build_output.stderr)
            );
        }
        Err(e) => {
            println!("Problem running build:\n{:#?}", e);
            process::exit(1);
        }
    }

    println!("Executing program...");
    println!("Running: {}", config.run);

    let execution_output = process::Command::new(config.run.as_str())
        .stdin(fs::File::open(config.input.as_str()).unwrap())
        .output()
        .expect("failed to execute process");

    let expected_output = fs::read_to_string(&config.output)
        .unwrap()
        .trim()
        .to_string();
    let execution_error = String::from_utf8_lossy(&execution_output.stderr)
        .trim()
        .to_string();
    let execution_output = String::from_utf8_lossy(&execution_output.stdout)
        .trim()
        .to_string();

    if execution_output == expected_output {
        println!("Program output matches expected output");
    } else {
        println!("Program output does not match expected output");
        println!("Execution errors:");
        println!("{}", execution_error);
        println!("Expected output:");
        println!("{}", expected_output);
        println!("Actual output:");
        println!("{}", execution_output);
        println!("Diff:");
        println!("{}", diff_lines(&expected_output, &execution_output));
    }
}
