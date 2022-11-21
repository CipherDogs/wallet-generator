use cyber::mnemonic::*;
use cyber::wallet::{PrivateKeyWallet, PublicKeyWallet};

/// Address struct
pub struct CyberAddress {
    pub mnemonic: String,
    pub cyber_address: String,
    pub bostrom_address: String,
    pub pussy_address: String,
}

/// Generate cyber address
pub fn generate() -> CyberAddress {
    let phrase = generate_phrase();

    let sk = PrivateKeyWallet::from_seed((*phrase).to_string(), None);
    let pk = PublicKeyWallet::from_private_key(sk.to_secret_key());

    CyberAddress {
        mnemonic: phrase,
        cyber_address: pk.to_address("cyber"),
        bostrom_address: pk.to_address("bostrom"),
        pussy_address: pk.to_address("pussy"),
    }
}
