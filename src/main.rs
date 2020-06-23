use std::env;
use std::process;

mod lib;
use lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    println!(
        "Searching for \"{}\" in file \"{}\"",
        config.query, config.filename
    );

    if let Err(e) = lib::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
