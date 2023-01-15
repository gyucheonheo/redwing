
#[test]
fn it_runs_basic_unix_commands() {
    let args = vec![String::from("redwing"), String::from("./")];
    let mut commands: Vec<Vec<String>> = vec![
        vec![String::from("pwd")],
        vec![String::from("ls"), String::from("-l")],
        vec![String::from("echo"), String::from("Hello World")]
    ];
    
    for rest in commands.iter_mut() {
        let mut args_cloned = args.clone();
        args_cloned.append(rest);
        let config = redwing::Config::build(&args_cloned).unwrap();

        match redwing::status(config)() {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "failed to execute {}", e)
        }
    }

}