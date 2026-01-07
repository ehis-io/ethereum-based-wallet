use ethers::prelude::*;
use ethers::signers::LocalWallet;
use std::sync::Arc;
use anyhow::Result;


#[tokio::main]
async fn main()-> Result<()>{
//generate random wallet
    let wallet = LocalWallet::new(&mut rand::thread_rng());


    println!("Random Wallet Address: {}", wallet.address());


    //connect to Ethereum network
    let provider = Provider::<Http>::try_from("https://ethereum-rpc.publicnode.com")?;

    //attach wallet to provider
    let client = Arc::new( SignerMiddleware::new(provider, wallet));
    
    //read balance

    let balance = client.get_balance(client.address(),None).await?;
    println!("Wallet Balance: {} ETH", ethers::utils::format_ether(balance));

    //generate wallet from mnemonic

    Ok(())
}
