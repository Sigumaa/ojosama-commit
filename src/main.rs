use serde_json::json;
use std::collections::BTreeMap;
use std::{process::Command, str::FromStr};
#[derive(Debug)]
struct Arguments {
    message: String,
}

//コードが汚いので綺麗にする。
#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let args = parse_args();
    let message = args.message;

    let post_body = json!({ "Text": message });
    let client = reqwest::Client::new();
    let res = client
        .post("https://ojosama.herokuapp.com/api/ojosama")
        .json(&post_body)
        .send()
        .await?
        .text()
        .await?;
    let res_text: BTreeMap<String, String> = serde_json::from_str(&res).expect("error");
    let commit_message = res_text.get("Result").expect("error");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .spawn()
        .expect("failed to start `git commit`");

    Ok(())
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

//しぐま
