use crate::base_converter::hex_decode;
use crate::decrypter::single_char_xor_plaintext_decrypt;

/// Find the xor 'key' by 'scoring' plain text (char frequency, etc) and decrypt
///
pub fn solve() -> String {
    let encrypted =
        hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();

    let decrypted = single_char_xor_plaintext_decrypt(&encrypted).unwrap();
    String::from_utf8(decrypted).unwrap()
}
