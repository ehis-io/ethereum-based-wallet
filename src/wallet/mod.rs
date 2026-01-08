use crate::wallet;

pub mod chain;
pub mod derive;
pub mod mnemonics;

pub async fn run() -> anyhow::Result<()> {
    let mnemonic = mnemonics::create_mnemonics().unwrap();
    println!("Generated Mnemonic: {}", mnemonic);

    let wallet = wallet::derive::wallet_from_mnemonic(mnemonic, 0).unwrap();
    println!("Derived Wallet Address: {:?}", wallet);

    let balance = wallet::chain::print_balance(wallet).await?;

    println!("Wallet ready: {}", balance);

    Ok(())
}
