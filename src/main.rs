use clap::{Arg, App};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::{Rem, Add};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    // Parsing the given flags
    let matches = App::new("")
        .arg(Arg::with_name("file")
                 .long("file")
                 .takes_value(true)
                 .help("File with students"))
        .arg(Arg::with_name("numbilets")
                 .long("numbilets")
                 .takes_value(true)
                 .help("Number of variants"))
        .arg(Arg::with_name("seed")
                 .long("parameter")
                 .takes_value(true)
                 .help("Parameter for PRNG"))         
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("students.txt");

    let numbilets: u32 = match matches.value_of("numbilets") {
        Some(num) => match num.parse() {
            Ok(n) => n,
            Err(_) => 30,
        },
        None => 30,
    };

    let seed = matches.value_of("seed").unwrap_or("10");

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(myfile).unwrap();
    let reader = BufReader::new(file);

    // Help variable ONE
    let one = 1.to_biguint().unwrap();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        let mut data = String::from(&line[..]);
        data.push_str(seed);

        let mut hasher = Sha256::new();
        hasher.input_str(&data); 

        let hashed = &mut [0u8; 32];
        hasher.result(hashed);

        let variant = BigUint::from_bytes_be(hashed)
            .rem(numbilets)
            .add(&one);    

        println!("{}:  {}", line, variant);
    }
}
