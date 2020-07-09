use crate::base_converter::base64_encode;
use crate::base_converter::hex_decode;

/// Convert hex to base64
///
pub fn solve() -> String {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let decoded = hex_decode(hex).unwrap();
    base64_encode(&decoded)
}
