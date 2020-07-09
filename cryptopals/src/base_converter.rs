use std::error::Error;
use std::fmt;

static HEX_LOOKUP: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

static BASE64_LOOKUP: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
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

/// Convert bytes to base64 encoded string
///
pub fn base64_encode(bytes: &[u8]) -> String {
    // Take 3 bytes and pack into single 24 bits
    // Grouping in six bit chunks convert the 24 bits to 4 byte values
    // Convert to base 64 using lookup table "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

    let aligned_len = bytes.len() / 3 * 3;
    let aligned_slice = &bytes[..aligned_len];
    let remaining_slice = &bytes[aligned_len..];

    //3 bytes generates 4, plus we need to pad so that the encoded data is aligned
    let output_padding = 3 - (remaining_slice.len() % 3);
    let encoded_len = ((bytes.len() + 2) / 3) * 4;
    let mut encoded_data = String::with_capacity(encoded_len);
    let mut block4: [u8; 4] = [0, 0, 0, 0];

    //Handle the 3 byte aligned partition
    for (i, _) in aligned_slice.iter().enumerate().step_by(3) {
        let packed = (((aligned_slice[i] as u32) << 16)
            | ((aligned_slice[i + 1] as u32) << 8)
            | aligned_slice[i + 2] as u32)
            & 0xFFFFFF;

        block4[0] = ((packed >> 18) & 0x3F) as u8;
        block4[1] = ((packed >> 12) & 0x3F) as u8;
        block4[2] = ((packed >> 6) & 0x3F) as u8;
        block4[3] = (packed & 0x3F) as u8;

        for b in block4.iter() {
            encoded_data.push(BASE64_LOOKUP[*b as usize]);
        }
    }

    if !remaining_slice.is_empty() {
        //Handle any remainder that requires padding
        let mut block3: [u8; 3] = [0, 0, 0];
        for (i, b) in remaining_slice.iter().enumerate() {
            block3[i] = *b;
        }

        let packed =
            (((block3[0] as u32) << 16) | ((block3[1] as u32) << 8) | block3[2] as u32) & 0xFFFFFF;

        block4[0] = ((packed >> 18) & 0x3F) as u8;
        block4[1] = ((packed >> 12) & 0x3F) as u8;
        block4[2] = ((packed >> 6) & 0x3F) as u8;
        block4[3] = (packed & 0x3F) as u8;

        for i in 0..block4.len() - output_padding {
            encoded_data.push(BASE64_LOOKUP[block4[i] as usize]);
        }

        for _ in 0..output_padding {
            encoded_data.push('=');
        }
    }

    encoded_data
}

/// Unit tests
///
#[test]
fn test_hex_decode_odd_length() {
    let decoded = hex_decode("FF0");
    assert!(decoded.is_err());
    let error = match decoded {
        Err(e) => e,
        Ok(_) => panic!("Shouldn't get here"),
    };
    assert_eq!(error, DecodeError::OddInputLength);
}

#[test]
fn test_hex_decode_invalid_char() {
    let decoded = hex_decode("!!00ZZ");
    assert!(decoded.is_err());
    let error = match decoded {
        Err(e) => e,
        Ok(_) => panic!("Shouldn't get here"),
    };
    assert_eq!(error, DecodeError::InvalidChar);
}

#[test]
fn test_hex_decode_valid() {
    let decoded = hex_decode("FF00FF");
    assert!(decoded.is_ok());
    let result = match decoded {
        Ok(bytes) => bytes,
        Err(_) => panic!("Shouldn't get here"),
    };
    assert_eq!(result, [255, 0, 255]);
}

#[test]
fn test_hex_encode_valid() {
    let encoded = hex_encode(&[255u8, 0u8, 255u8]);
    assert_eq!(encoded, "FF00FF");
}

#[test]
fn test_base64_encode_aligned() {
    let encoded_string = base64_encode("Build me up buttercup".as_bytes());
    assert_eq!(encoded_string, "QnVpbGQgbWUgdXAgYnV0dGVyY3Vw");
}

#[test]
fn test_base64_encode_unaligned() {
    let encoded_string =
        base64_encode("Why do you build me up buttercup baby just to let me down?".as_bytes());
    assert_eq!(
        encoded_string,
        "V2h5IGRvIHlvdSBidWlsZCBtZSB1cCBidXR0ZXJjdXAgYmFieSBqdXN0IHRvIGxldCBtZSBkb3duPw=="
    );
}
