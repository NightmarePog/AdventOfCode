use dotenvy::dotenv;
use reqwest::Client;
use std::env;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

pub async fn get_http(id: u16) -> Result<String, reqwest::Error> {
    let client = Client::new();
    dotenv().ok();
    let session = env::var("SESSION").expect("SESSION not found");
    let res = client
        .get(format!("https://adventofcode.com/2025/day/{}/input", id))
        .header("Cookie", format!("session={}", session))
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    Ok(body)
}

pub async fn get_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
