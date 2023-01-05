
pub struct Config {
    pub path: String,
    pub cmd: Vec<String>
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
//        if args.len() < 3 {
//            return Err("Not enough arguments!");
//        }
        let path = args[1].clone();
        let cmd: Vec<String> = (&args[2..])
            .to_vec();

        Ok(Config { path, cmd })
    }
}
