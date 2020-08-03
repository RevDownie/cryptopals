#![feature(iter_map_while)]

mod base64_converter;
mod hamming;
mod hex_converter;
mod padding;
mod set1;
mod set2;
mod xor_decrypter;
mod xor_encrypter;

/// Cryptopals challenges https://cryptopals.com
///
fn main() {
    println!("=== Set 1");
    println!("\t{}", set1::challenge1::solve());
    println!("\t{}", set1::challenge2::solve());
    println!("\t{}", set1::challenge3::solve());
    println!("\t{}", set1::challenge4::solve().replace('\n', ""));
    println!("\t{}", set1::challenge5::solve());
    println!("\t{}", set1::challenge6::solve().replace('\n', ""));
    println!("\t{}", set1::challenge7::solve().replace('\n', ""));
    println!("\t{}", set1::challenge8::solve());

    println!("=== Set 2");
    println!("\t{}", set2::challenge9::solve());

    println!("=== Finished");
}
