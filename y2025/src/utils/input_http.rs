use dotenvy::dotenv;
use reqwest::Client;
use std::env;

pub async fn get(id: u16) -> Result<Vec<String>, reqwest::Error> {
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
    let lines: Vec<String> = body.lines().map(|s| s.to_string()).collect();

    Ok(lines)
}
