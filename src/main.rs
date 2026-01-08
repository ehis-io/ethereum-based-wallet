mod wallet;
#[tokio::main]

async fn main() {
    println!("Creating wallet...");
    wallet::run().await.unwrap();
}
