#![feature(iter_map_while)]

mod base_converter;
mod decrypter;
mod set1;

/// Cryptopals challenges https://cryptopals.com
///
fn main() {
    println!("=== Set 1");
    println!("\t{}", set1::challenge1::solve());
    println!("\t{}", set1::challenge2::solve());
    println!("\t{}", set1::challenge3::solve());
    println!("\t{}", set1::challenge4::solve());
    println!("=== Finished");
}
