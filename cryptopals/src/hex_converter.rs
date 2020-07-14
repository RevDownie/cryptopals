use std::error::Error;
use std::fmt;

static HEX_LOOKUP: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DecodeError {
    OddInputLength,
    InvalidChar,
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::OddInputLength => "Hex string contains odd number of characters",
            DecodeError::InvalidChar => "Hex string contains invalid character -  should be [0-F]",
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::OddInputLength => {
                write!(f, "Hex string contains odd number of characters")
            }
            DecodeError::InvalidChar => write!(
                f,
                "Hex string contains invalid character -  should be [0-F]"
            ),
        }
    }
}

/// Convert hex to bytes
///
pub fn hex_decode(hex: &str) -> std::result::Result<Vec<u8>, DecodeError> {
    if hex.len() % 2 != 0 {
        Err(DecodeError::OddInputLength)
    } else {
        let expected_len = hex.len() / 2;
        let mut digits = hex.chars().filter_map(|c| c.to_digit(16)).fuse();
        let mut bytes: Vec<u8> = Vec::with_capacity(expected_len);
        while let (Some(h), Some(l)) = (digits.next(), digits.next()) {
            bytes.push((h << 4 | l) as u8)
        }
        if bytes.len() == expected_len {
            Ok(bytes)
        } else {
            Err(DecodeError::InvalidChar)
        }
    }
}

/// Convert bytes to hex
///
pub fn hex_encode(bytes: &[u8]) -> String {
    let encoded_len = bytes.len() * 2;
    let mut encoded_data = String::with_capacity(encoded_len);
    for b in bytes.iter() {
        encoded_data.push(HEX_LOOKUP[(*b >> 4) as usize]);
        encoded_data.push(HEX_LOOKUP[(*b & 0x0F) as usize]);
    }

    encoded_data
}

/// Unit tests
///
#[test]
fn test_hex_decode_odd_length() {
    let decoded = hex_decode("FF0");
    assert!(decoded.is_err());
    assert_eq!(decoded.unwrap_err(), DecodeError::OddInputLength);
}

#[test]
fn test_hex_decode_invalid_char() {
    let decoded = hex_decode("!!00ZZ");
    assert!(decoded.is_err());
    assert_eq!(decoded.unwrap_err(), DecodeError::InvalidChar);
}

#[test]
fn test_hex_decode_valid() {
    let decoded = hex_decode("FF00FF");
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap(), [255, 0, 255]);
}

#[test]
fn test_hex_encode_valid() {
    let encoded = hex_encode(&[255u8, 0u8, 255u8]);
    assert_eq!(encoded, "FF00FF");
}
