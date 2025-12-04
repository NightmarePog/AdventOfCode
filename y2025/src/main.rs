pub mod solutions;
pub mod utils;

#[tokio::main]
async fn main() {
    solutions::one::init().await;
    println!("-------------------------------");
    solutions::two::init().await;
}
