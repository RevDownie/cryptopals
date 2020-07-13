#![feature(iter_map_while)]

mod base_converter;
mod set1;
mod xor_decrypter;
mod xor_encrypter;

/// Cryptopals challenges https://cryptopals.com
///
fn main() {
    println!("=== Set 1");
    println!("\t{}", set1::challenge1::solve());
    println!("\t{}", set1::challenge2::solve());
    println!("\t{}", set1::challenge3::solve());
    println!("\t{}", set1::challenge4::solve());
    println!("\t{}", set1::challenge5::solve());
    println!("=== Finished");
}
