use crate::hex_converter::hex_decode;
use crate::xor_decrypter::single_char_xor_plaintext_decrypt;
use std::fs::File;
use std::io::{self, BufRead};

/// Find the xor 'key' by 'scoring' plain text (char frequency, etc) and decrypt
///
pub fn solve() -> String {
    let payload_file = File::open("payloads/set1/challenge4.txt").unwrap();
    let encoded_lines = io::BufReader::new(payload_file).lines();

    let best_decrypted = encoded_lines
        .filter_map(|e| single_char_xor_plaintext_decrypt(&hex_decode(&e.unwrap()).unwrap()))
        .max_by_key(|(_, s, _)| *s);

    let (text, _, _) = best_decrypted.unwrap();
    String::from_utf8(text).unwrap()
}
