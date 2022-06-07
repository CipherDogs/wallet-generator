use data_encoding::HEXLOWER;
use sp_core::crypto::AccountId32;
use sp_core::crypto::Ss58AddressFormat;
use sp_core::crypto::Ss58Codec;
use sp_core::Pair;

/// Address struct
pub struct PolkadotAddress {
    pub mnemonic_phrase: String,
    pub mini_secret_key: String,
    pub public_key: String,
    pub address: String,
}

/// Generate Polkadot address
pub fn generate(ss58format: u8) -> PolkadotAddress {
    let (pair, phrase, secret) = sp_core::sr25519::Pair::generate_with_phrase(None);
    let address = AccountId32::from(pair.public())
        .to_ss58check_with_version(Ss58AddressFormat::custom(ss58format.into()));
    PolkadotAddress {
        mnemonic_phrase: phrase,
        mini_secret_key: HEXLOWER.encode(&secret),
        public_key: HEXLOWER.encode(&<[u8; 32]>::from(pair.public())),
        address,
    }
}
