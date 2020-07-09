use crate::base_converter::hex_decode;
use crate::base_converter::hex_encode;

/// Convert hex to base64
///
pub fn solve() -> String {
    let lhs = hex_decode("1c0111001f010100061a024b53535009181c").unwrap();
    let rhs = hex_decode("686974207468652062756c6c277320657965").unwrap();
    let xord: Vec<u8> = lhs.iter().zip(rhs.iter()).map(|(a, b)| a ^ b).collect();
    hex_encode(&xord)
}
