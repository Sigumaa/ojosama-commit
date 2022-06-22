use std::{process::Command, str::FromStr};

#[derive(Debug)]
struct Arguments {
    message: String,
}

fn main() {
    let args = parse_args();
    let commit_message = args.message;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .spawn()
        .expect("failed to start `git commit`");
}

fn parse_args() -> Arguments {
    let mut args = Vec::new();

    for arg in std::env::args().skip(1) {
        args.push(String::from_str(&arg).expect("error parsing arguments"));
    }

    if args.len() != 1 {
        eprintln!(
            "Error: wrong number of argsments: expected 1, got {}.",
            args.len()
        );
        eprintln!(r#"Try using """#);
        std::process::exit(1);
    };

    Arguments {
        message: args[0].to_string(),
    }
}
