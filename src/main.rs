use clap::{App, Arg};

mod cyber_generator;

fn main() {
    let matches = App::new("walletgen")
        .version("0.2.0")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("Generator of addresses and mnemonic phrases for blockchains")
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
        let acc = cyber_generator::generate();

        println!("# {}", i + 1);
        println!("address: {}", acc.address);
        println!("mnemonic: {}", acc.mnemonic);
        println!("---------------------------------------------------");
    }
}
