use acryl::account::PrivateKeyAccount;
use acryl::seed::*;

pub struct Address {
    pub seed: String,
    pub address: String,
    pub public_key: String,
    pub private_key: String,
}

pub fn generate(chain_id: u8) -> Address {
    let phrase = generate_phrase();

    let account = PrivateKeyAccount::from_seed(&phrase);

    Address {
        seed: phrase,
        address: account.public_key().to_address(chain_id).to_string(),
        public_key: account.public_key().to_string(),
        private_key: account.to_string(),
    }
}
