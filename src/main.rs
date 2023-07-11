mod coin_generator;
mod cyber_generator;
mod polkadot_generator;

use clap::{Parser, ValueEnum};
use wallet_gen::coin::Coin;

#[derive(Debug, Clone, ValueEnum)]
enum Blockchain {
    Cyber,
    Bitcoin,
    Ethereum,
    Monero,
    Polkadot,
    Kusama,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    blockchain: Blockchain,
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let args = Args::parse();

    for i in 0..args.count {
        match args.blockchain {
            Blockchain::Cyber => {
                let acc = cyber_generator::generate();

                println!("# {}", i + 1);
                println!("cyber address: {}", acc.cyber_address);
                println!("bostrom address: {}", acc.bostrom_address);
                println!("pussy address: {}", acc.pussy_address);
                println!("mnemonic: {}", acc.mnemonic);
            }
            Blockchain::Bitcoin => {
                let acc = coin_generator::generate(Coin::Bitcoin);

                println!("# {}", i + 1);
                println!("address: {}", acc.address);
                println!("public key: {}", acc.public_key);
                println!("private key: {}", acc.private_key);
            }
            Blockchain::Ethereum => {
                let acc = coin_generator::generate(Coin::Ethereum);

                println!("# {}", i + 1);
                println!("address: {}", acc.address);
                println!("public key: 0x{}", acc.public_key);
                println!("private key: 0x{}", acc.private_key);
            }
            Blockchain::Monero => {
                let acc = coin_generator::generate(Coin::Monero);

                println!("# {}", i + 1);
                println!("address: {}", acc.address);
                println!("public key: {}", acc.public_key);
                println!("private key: {}", acc.private_key);
            }
            Blockchain::Polkadot => {
                let acc = polkadot_generator::generate(0_u8);

                println!("# {}", i + 1);
                println!("address: {}", acc.address);
                println!("mnemonic: {}", acc.mnemonic_phrase);
                println!("public key: {}", acc.public_key);
                println!("mini secret key: {}", acc.mini_secret_key);
            }
            Blockchain::Kusama => {
                let acc = polkadot_generator::generate(2_u8);

                println!("# {}", i + 1);
                println!("address: {}", acc.address);
                println!("mnemonic: {}", acc.mnemonic_phrase);
                println!("public key: {}", acc.public_key);
                println!("mini secret key: {}", acc.mini_secret_key);
            }
        }

        println!("---------------------------------------------------");
    }
}
