use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Serialize)]
struct Arguments {
    message: String,
}

#[derive(Debug)]
struct OjosamaError(String);

impl std::fmt::Display for OjosamaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "エラーが発生しましたわ。理由は以下の通りですわ。\n{}", self.0)
    }
}

impl std::error::Error for OjosamaError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    let commit_message = send_request(&args).await?;
    commit_to_git(&commit_message)?;
    
    Ok(())
}

fn parse_args() -> Result<Arguments, Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        let error_message = format!(
            "引数の数が正しくありませんわ。引数の数は1つである必要がありますわ。与えられた引数は{}個でしたわ。",
            args.len()
        );
        return Err(Box::new(OjosamaError(error_message)));
    };

    let message = args[0].clone();

    Ok(Arguments { message })
}

async fn send_request(args: &Arguments) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.ojosama.jiro4989.com")
        .header("Content-Type", "application/json")
        .json(&json!({
            "Text": args.message,
        }))
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    let res_text: HashMap<String, String> = serde_json::from_str(&res)?;

    let commit_message = res_text
        .get("Result")
        .ok_or_else(|| OjosamaError("APIからかえってきたデータに異常がありましたわ。".to_string()))?
        .clone();

    Ok(commit_message)
}

fn commit_to_git(commit_message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let is_success = Command::new("git")
        .args(&["commit", "-m", commit_message])
        .spawn()?
        .wait()?
        .success();

    if !is_success {
        return Err(Box::new(OjosamaError(
            "`git commit` に失敗してしまいましたわ。addはしましたの？".to_string(),
        )));
    }

    Ok(())
}
