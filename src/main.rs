use std::env;
use std::process;
use lilgrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("PROGRAM ERROR: Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("PROGRAM ERROR: Application error: {error}");
        process::exit(1);
    }
}