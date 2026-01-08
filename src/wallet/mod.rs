use crate::wallet;

pub mod derive;
pub mod mnemonics;

pub fn run() {
    let mnemonic = mnemonics::create_mnemonics().unwrap();
    println!("Generated Mnemonic: {}", mnemonic);

    let wallet = wallet::derive::wallet_from_mnemonic(mnemonic, 0).unwrap();
    println!("Derived Wallet Address: {:?}", wallet);

    println!("Wallet ready");
}
