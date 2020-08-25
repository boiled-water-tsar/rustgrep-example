mod logic;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = logic::Config::new(&args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
        });

    if let Err(e) = logic::grep_logic(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
