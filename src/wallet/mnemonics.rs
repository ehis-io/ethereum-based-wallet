use anyhow::{Ok, Result};
use ethers::prelude::*;
use ethers::signers::coins_bip39::{English, Mnemonic};
use rand::thread_rng; // RNG

pub fn create_mnemonics() -> Result<String> {
    let mut rng = thread_rng();

    let mnemonic = Mnemonic::<English>::new(&mut rng);

    let phrase = mnemonic.to_phrase().to_string();

    Ok(phrase)
}
