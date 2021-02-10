use wallet_gen::coin::Coin;
use wallet_gen::{bitcoin, cryptonote, ethereum};

/// Address struct
pub struct CoinAddress {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
}

/// Generate Bitcoin, Ethereum, Monero address
pub fn generate(coin: Coin) -> CoinAddress {
    let wallet;

    match coin {
        Coin::Ethereum => {
            wallet = ethereum::new_wallet(Coin::Ethereum).expect("Error generating address")
        }
        Coin::Monero => {
            wallet = cryptonote::new_wallet(Coin::Monero).expect("Error generating address")
        }
        _ => wallet = bitcoin::new_wallet(Coin::Bitcoin).expect("Error generating address"),
    }

    CoinAddress {
        address: wallet.address,
        public_key: wallet.public_key,
        private_key: wallet.private_key,
    }
}
