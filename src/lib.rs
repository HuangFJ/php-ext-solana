#![cfg_attr(windows, feature(abi_vectorcall))]
use bip39::{Mnemonic, MnemonicType, Seed};
use ext_php_rs::{boxed::ZBox, prelude::*, types::ZendObject};
use solana_sdk::{
    derivation_path::DerivationPath, signature::keypair_from_seed_and_derivation_path,
    signer::Signer,
};

#[php_function]
pub fn sol_create_wallet() -> ZBox<ZendObject> {
    let mnemonic_type = MnemonicType::for_word_count(12).unwrap();
    let mnemonic = Mnemonic::new(mnemonic_type, bip39::Language::English);
    // let mnemonic = Mnemonic::from_phrase("street biology soda own fortune debris matter ankle trend inside good reject", bip39::Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    let keypair = keypair_from_seed_and_derivation_path(
        seed.as_bytes(),
        DerivationPath::from_key_str("0'/0'").ok(),
    )
    .unwrap();
    let phrase: &str = mnemonic.phrase();

    let mut obj = ZendObject::new_stdclass();
    let _ = obj.set_property("phrase", phrase);
    let _ = obj.set_property("address", keypair.pubkey().to_string());

    obj
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
