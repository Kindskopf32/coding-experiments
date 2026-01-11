use reqwest::{header::{HeaderMap, HeaderValue}};
use anyhow::{Context};
use clap::Parser;
use std::env;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "URL", help = "URL of PR")]
    url: String,

    #[arg(short, long, value_name = "ISSUE", help = "Output help")]
    pr: Option<String>,
}

//#[derive(Debug)]
//struct Config {
//    gitea_token: String,
//    openrouter_token: String,
//
//}
//
//impl Config {
//    fn from_env() -> Result<Self, Box<dyn Error>> {
//        let gitea_token = env::var("GITEA_TOKEN")?;
//        let openrouter_token = env::var("OPENROUTER_TOKEN")?;
//
//        Ok(Config { gitea_token, openrouter_token })
//    }
//}

async fn get_diff(url: &str, token: &str) -> anyhow::Result<String> {
    let auth_header = HeaderValue::from_str(&format!("token {}", token)).context("Failed to format token into header value")?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_header);

    let client = reqwest::Client::builder().default_headers(headers).build()?;

    let response = client.get(url).send().await?.error_for_status();
    let _response2 = client.post(url).body("Text").send().await?.error_for_status();
    let body = response?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let gitea_token = match env::var("GITEA_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Missing GITEA_TOKEN environment variable");
            std::process::exit(1);
        }
    };
    let _openrouter_token = match env::var("OPENROUTER_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Missing OPENROUTER_TOKEN environment variable");
            std::process::exit(1);
        }
    };

    let args = Args::parse();

    match get_diff(&args.url, &gitea_token).await {
        Ok(body) => println!("Function successful got \n{}", body),
        Err(e) => println!("Error {}", e),
    }
}
