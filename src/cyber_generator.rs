use cyber::mnemonic::*;
use cyber::wallet::{PrivateKeyWallet, PublicKeyWallet};

/// Address struct
pub struct CyberAddress {
    pub mnemonic: String,
    pub address: String,
}

/// Generate cyber address
pub fn generate() -> CyberAddress {
    let phrase = generate_phrase();

    let sk = PrivateKeyWallet::from_seed((*phrase).to_string(), None);
    let pk = PublicKeyWallet::from_private_key(sk.to_secret_key());

    CyberAddress {
        mnemonic: phrase,
        address: pk.to_address(),
    }
}
