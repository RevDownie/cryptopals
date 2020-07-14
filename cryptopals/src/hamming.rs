use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HammingError {
    MismatchedLengths,
}

impl Error for HammingError {
    fn description(&self) -> &str {
        match *self {
            HammingError::MismatchedLengths => {
                "Data to compute distance between must be same length"
            }
        }
    }
}

impl fmt::Display for HammingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HammingError::MismatchedLengths => {
                write!(f, "Data to compute distance between must be same length")
            }
        }
    }
}

/// Calculate the bitwise hamming distance between 2 same sized byte arrays
///
pub fn bitwise_hamming_dist(lhs: &[u8], rhs: &[u8]) -> std::result::Result<u32, HammingError> {
    if lhs.len() != rhs.len() {
        return Err(HammingError::MismatchedLengths);
    }

    let distance = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(a, b)| num_set_bits(a ^ b))
        .sum();

    Ok(distance)
}

/// Counts the number of set bits in a byte
///
fn num_set_bits(byte: u8) -> u32 {
    let mut count = 0;
    let mut x = byte;
    while x > 0 {
        count += x & 1;
        x >>= 1;
    }

    count as u32
}

/// Unit tests
///
#[test]
fn test_hamming_distance_bitwise_valid() {
    let result = bitwise_hamming_dist(b"this is a test", b"wokka wokka!!!");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 37);
}

#[test]
fn test_hamming_distance_bitwise_mismatched_length() {
    let result = bitwise_hamming_dist(b"this is a test", b"wokka wokka");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), HammingError::MismatchedLengths);
}
