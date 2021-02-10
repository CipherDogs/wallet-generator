use clap::{App, Arg, SubCommand};
use wallet_gen::coin::Coin;

mod coin_generator;
mod cyber_generator;
mod polkadot_generator;

fn main() {
    let matches = App::new("walletgen")
        .version("0.2.0")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("Generator of addresses and mnemonic phrases for blockchains")
        .subcommand(SubCommand::with_name("cyber").about("cyber blockchain"))
        .subcommand(SubCommand::with_name("bitcoin").about("Bitcoin blockchain"))
        .subcommand(SubCommand::with_name("ethereum").about("Ethereum blockchain"))
        .subcommand(SubCommand::with_name("monero").about("Monero blockchain"))
        .subcommand(SubCommand::with_name("polkadot").about("Polkadot blockchain"))
        .subcommand(SubCommand::with_name("kusama").about("Kusama blockchain"))
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("INT")
                .help("Sets a address generate count"),
        )
        .get_matches();

    let count_arg = matches.value_of("count").unwrap_or("1");
    let count = count_arg.parse().unwrap();

    for i in 0..count {
        if let Some(_) = matches.subcommand_matches("cyber") {
            let acc = cyber_generator::generate();

            println!("# {}", i + 1);
            println!("address: {}", acc.address);
            println!("mnemonic: {}", acc.mnemonic);
        } else if let Some(_) = matches.subcommand_matches("bitcoin") {
            let acc = coin_generator::generate(Coin::Bitcoin);

            println!("# {}", i + 1);
            println!("address: {}", acc.address);
            println!("public key: {}", acc.public_key);
            println!("private key: {}", acc.private_key);
        } else if let Some(_) = matches.subcommand_matches("ethereum") {
            let acc = coin_generator::generate(Coin::Ethereum);

            println!("# {}", i + 1);
            println!("address: {}", acc.address);
            println!("public key: 0x{}", acc.public_key);
            println!("private key: 0x{}", acc.private_key);
        } else if let Some(_) = matches.subcommand_matches("monero") {
            let acc = coin_generator::generate(Coin::Monero);

            println!("# {}", i + 1);
            println!("address: {}", acc.address);
            println!("public key: {}", acc.public_key);
            println!("private key: {}", acc.private_key);
        } else if let Some(_) = matches.subcommand_matches("polkadot") {
            let acc = polkadot_generator::generate(0 as u8);

            println!("# {}", i + 1);
            println!("address: {}", acc.address);
            println!("mnemonic: {}", acc.mnemonic_phrase);
            println!("public key: {}", acc.public_key);
            println!("mini secret key: {}", acc.mini_secret_key);
        } else if let Some(_) = matches.subcommand_matches("kusama") {
            let acc = polkadot_generator::generate(2 as u8);

            println!("# {}", i + 1);
            println!("address: {}", acc.address);
            println!("mnemonic: {}", acc.mnemonic_phrase);
            println!("public key: {}", acc.public_key);
            println!("mini secret key: {}", acc.mini_secret_key);
        } else {
            break;
        }

        println!("---------------------------------------------------");
    }
}
