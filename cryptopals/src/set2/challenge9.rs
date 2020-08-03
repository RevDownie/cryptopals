use crate::hex_converter::hex_encode;
use crate::padding::padded;

/// Pad data to a fixed alignment
///
pub fn solve() -> String {
    let data = b"YELLOW SUBMARINE";
    let p = padded(&data[..], 20);
    hex_encode(&p)
}
