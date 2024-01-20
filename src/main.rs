use rustygrep::InitialConfig;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let initial_config = InitialConfig::build(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    if let Err(e) = rustygrep::run(initial_config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
