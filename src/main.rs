use clap::{App, Arg};

mod generator;

use generator::*;

fn main() {
    let matches = App::new("walletgen")
        .version("0.1.2")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("CLI address generator for Acryl or Waves")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("INT")
                .help("Sets a address generate count"),
        )
        .arg(
            Arg::with_name("chainID")
                .short("i")
                .long("chain")
                .value_name("CHAR")
                .help("Sets a chainID blockchain"),
        )
        .get_matches();

    let count_arg = matches.value_of("count").unwrap_or("1");
    let count = count_arg.parse().unwrap();

    let chain_id_arg = matches.value_of("chainID").unwrap_or("A");
    let chain_id = chain_id_arg.as_bytes();

    for i in 0..count {
        let acc = generate(chain_id[0] as u8);

        println!("# {}", i + 1);
        println!("address: {}", acc.address);
        println!("public key: {}", acc.public_key);
        println!("private key: {}", acc.private_key);
        println!("seed: {}", acc.seed);
        println!("---------------------------------------------------");
    }
}
