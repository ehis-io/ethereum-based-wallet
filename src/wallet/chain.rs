use anyhow::Result;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use std::sync::Arc;

pub async fn print_balance(wallet: LocalWallet) -> Result<U256> {
    let provider = Provider::<Http>::try_from("https://ethereum-rpc.publicnode.com").unwrap();
    let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));
    let balance = client.get_balance(wallet.address(), None).await?;

    println!("Wallet Balance: {}", balance);

    Ok(balance)
}
