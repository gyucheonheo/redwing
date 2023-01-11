use std::env;

use redwing::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    redwing::run(_config).unwrap_or_else(|err| {
        eprintln!("reding failed to run: {}", err);
        std::process::exit(1);
    });
}
