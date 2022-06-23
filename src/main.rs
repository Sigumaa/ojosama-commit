use serde_json::json;
use std::collections::HashMap;
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
    let res_text: HashMap<String, String> =
        serde_json::from_str(&res).expect("帰ってきたデータの処理に失敗してしまいましたわ。");

    let commit_message = res_text
        .get("Result")
        .expect("メッセージを取り出すのに失敗してしまいましたわ。");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .spawn()
        .expect("`git commit`に失敗してしまいましたわ。");

    Ok(())
}

fn parse_args() -> Arguments {
    let mut args = Vec::new();

    for arg in std::env::args().skip(1) {
        args.push(String::from_str(&arg).expect("引数のパースに失敗してしまいましたわ。"));
    }

    if args.len() != 1 {
        eprintln!(
            "引数の数が間違っておりますわ。　引数の数は1個であってほしいですわ。　実際の引数の数は {} 個でしたわ。",
            args.len()
        );
        eprintln!(r#"ダブルクオーテーション "" で メッセージを囲ってみてくださいまし。"#);
        std::process::exit(1);
    };

    Arguments {
        message: args[0].to_string(),
    }
}
