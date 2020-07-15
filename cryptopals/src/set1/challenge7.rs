use crate::base64_converter::base64_decode;
use openssl::symm::Cipher;
use std::fs;

/// Decrypt the base64 encoded file using AES-128 ECB key
///
pub fn solve() -> String {
    let encoded = fs::read_to_string("payloads/set1/challenge7.txt").unwrap();
    let decoded = base64_decode(&encoded).unwrap();

    let key = b"YELLOW SUBMARINE";
    let cipher = Cipher::aes_128_ecb();
    let decrypted = openssl::symm::decrypt(cipher, &key[..], None, &decoded).unwrap();
    String::from_utf8(decrypted).unwrap()
}
