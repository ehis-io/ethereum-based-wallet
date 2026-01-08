use anyhow::Result;

use ethers::signers::coins_bip39::English;
use ethers::signers::{LocalWallet, MnemonicBuilder};

pub fn wallet_from_mnemonic(mnemonic: String, index: u32) -> Result<LocalWallet> {
    let phrase = mnemonic;

    let builder = MnemonicBuilder::<English>::default().phrase(phrase.as_str());

    let builder = builder.index(index)?;

    let wallet: LocalWallet = builder.build()?;

    let wallet_clone = wallet.clone();

    // let provider = Provider::<Http>::try_from("https://ethereum-rpc.publicnode.com")?;
    // let client = Arc::new(SignerMiddleware::new(provider, wallet));

    Ok(wallet_clone)
}
