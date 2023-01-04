use std::env;
use fork::{daemon, Fork};
use std::process::Command;

use redwing::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _config = Config::build(&args).unwrap();

    if let Ok(Fork::Child) = daemon(false, false) {
       Command::new("sleep")
           .arg("300")
           .output()
           .expect("failed to execute process");
    }
}
