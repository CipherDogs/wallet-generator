use cyber::mnemonic::*;
use cyber::wallet::{PrivateKeyWallet, PublicKeyWallet};

pub struct Address {
    pub mnemonic: String,
    pub address: String,
}

pub fn generate() -> Address {
    let phrase = generate_phrase();

    let sk = PrivateKeyWallet::from_seed((*phrase).to_string(), None);
    let pk = PublicKeyWallet::from_private_key(sk.to_secret_key());

    Address {
        mnemonic: phrase,
        address: pk.to_address(),
    }
}
