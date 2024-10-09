use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        match first_arg.as_str() {
            "run" => run(&args[2..]),
            _ => panic!("{first_arg} is not a valid argument"),
        }
    } else {
        panic!("what?");
    }
}

fn run(args: &[String]) {
    if let Some(command) = args.get(0) {
        Command::new(command)
            .args(&args[1..])
            .status()
            .expect("command failed to start");
    } else {
        panic!("no command provided");
    }
}
