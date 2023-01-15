use std::process::{
    Command, ExitStatus
};

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

pub fn status(config: Config) -> impl FnMut() -> Result<ExitStatus, std::io::Error> { 
    let program = &config.cmd[0];
    let rest = &config.cmd[1..];
    let mut binding = Command::new(program);
    
    binding.current_dir(config.path);
    binding.args(rest);

    move|| binding.status()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn building_config_would_return_error_if_arguments_are_not_enough() {
        let empty_args : Vec<String> = vec![];
        let config = Config::build(&empty_args);

        match config {
            Ok(_) => assert!(false, "building Config should fail with empty args"),
            Err(_) => assert!(true)
        };

        let one_args: Vec<String> = vec![String::from("test")];
        let config1= Config::build(&one_args);

        match config1 {
            Ok(_) => assert!(false, "building Config should fail with length 1 args"),
            Err(_) => assert!(true)
        };

        let two_args: Vec<String> = vec![String::from("test"), String::from("test1")];
        let config2= Config::build(&two_args);

        match config2 {
            Ok(_) => assert!(false, "building Config should fail with length 2 args"),
            Err(_) => assert!(true)
        };
    }

    #[test]
    fn building_config_would_return_config_if_arguments_are_enough() {
        let args : Vec<String> = vec![String::from("test"), String::from("test1"), String::from("test2")];
        let config = Config::build(&args);

        match config {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "building Config should build successfully with len 3 args")
        };
    }
}
