use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::process::Command;

const API_ENDPOINT: &str = "https://api.ojosama.jiro4989.com";

#[derive(Debug)]
struct Arguments {
    message: String,
}

#[derive(Debug, Serialize)]
struct OjosamaRequest<'a> {
    #[serde(rename = "Text")]
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct OjosamaResponse {
    #[serde(rename = "Result")]
    result: String,
}

#[derive(Debug)]
enum OjoError {
    InvalidArgumentCount { provided: usize },
    Api(reqwest::Error),
    ApiResponseMissing,
    GitIo(std::io::Error),
    GitFailure,
}

impl fmt::Display for OjoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OjoError::InvalidArgumentCount { provided } => write!(
                f,
                "引数の数が正しくありませんわ。引数は1つである必要がございますのに、{}個いただきましたわ。",
                provided
            ),
            OjoError::Api(source) => write!(
                f,
                "Ojosama APIへのアクセスで問題が発生しましたわ: {}",
                source
            ),
            OjoError::ApiResponseMissing => write!(
                f,
                "APIからお返事が届きませんでしたわ。少し間を置いて再度お試しくださいまし。"
            ),
            OjoError::GitIo(source) => write!(
                f,
                "gitコマンドの実行中に入出力エラーが発生いたしましたわ: {}",
                source
            ),
            OjoError::GitFailure => write!(
                f,
                "`git commit` に失敗してしまいましたわ。`git add` はお済みですの？"
            ),
        }
    }
}

impl std::error::Error for OjoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OjoError::Api(source) => Some(source),
            OjoError::GitIo(source) => Some(source),
            _ => None,
        }
    }
}

type Result<T> = std::result::Result<T, OjoError>;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = parse_args()?;
    let commit_message = request_ojosama_message(&args.message).await?;
    commit_to_git(&commit_message)?;
    Ok(())
}

fn parse_args() -> Result<Arguments> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.as_slice() {
        [message] => Ok(Arguments {
            message: message.clone(),
        }),
        _ => Err(OjoError::InvalidArgumentCount {
            provided: args.len(),
        }),
    }
}

async fn request_ojosama_message(message: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .post(API_ENDPOINT)
        .json(&OjosamaRequest { text: message })
        .send()
        .await
        .map_err(OjoError::Api)?
        .json::<OjosamaResponse>()
        .await
        .map_err(OjoError::Api)?;

    let trimmed = response.result.trim();
    if trimmed.is_empty() {
        return Err(OjoError::ApiResponseMissing);
    }

    Ok(trimmed.to_string())
}

fn commit_to_git(commit_message: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .status()
        .map_err(OjoError::GitIo)?;

    if status.success() {
        Ok(())
    } else {
        Err(OjoError::GitFailure)
    }
}
