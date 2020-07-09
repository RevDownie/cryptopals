mod base_converter;
mod set1;

/// Cryptopals challenges https://cryptopals.com
///
fn main() {
    println!("=== Set 1");
    println!("\t{}", set1::challenge1::solve());
    println!("\t{}", set1::challenge2::solve());
}
