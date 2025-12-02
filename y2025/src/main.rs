pub mod solutions;
pub mod utils;
use solutions::one;

#[tokio::main]
async fn main() {
    one::init().await;
}
