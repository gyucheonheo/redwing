use std::{
    env,
    process::Command
};

use redwing::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _config = Config::build(&args).unwrap();
    let program = &_config.cmd[0];
    let rest = &_config.cmd[1..];
    let mut _cmd = Command::new(program);

    _cmd.current_dir(_config.path);

    if rest.len() == 0 {
        _cmd.status().expect("process failed to execute");
    } else {
        _cmd.args(rest).status().expect("process failed to execute");
    }
}
