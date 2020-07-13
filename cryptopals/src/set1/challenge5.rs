use crate::base_converter::hex_encode;
use crate::xor_encrypter::xor_encrypt;

/// Repeating XOR key
///
pub fn solve() -> String {
    let encrypted = xor_encrypt(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        "ICE",
    );
    hex_encode(&encrypted)
}
