use std::process::Command;
pub struct Config {
    pub path: String,
    pub cmd: Vec<String>
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let path = args[1].clone();
        let cmd: Vec<String> = (&args[2..])
            .to_vec();

        Ok(Config { path, cmd })
    }
}

pub fn run(config: Config) -> Result<std::process::ExitStatus, std::io::Error>{
    let program = &config.cmd[0];
    let rest = &config.cmd[1..];
    let mut binding = Command::new(program);
    
    binding.current_dir(config.path);

    let status = match rest {
        [] => binding.status(),
        otherwise => binding.args(otherwise).status(),
    };

    status
}
