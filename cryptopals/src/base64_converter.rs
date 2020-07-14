use std::error::Error;
use std::fmt;

static BASE64_LOOKUP: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

//Mapping ascii range to base 64 range
static BASE64_REV_LOOKUP: &[u32] = &[
    62, 0xff, 0xff, 0xff, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 0xff, 0xff, 0xff, 0, 0xff,
    0xff, 0xff, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
    23, 24, 25, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
    38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DecodeError {
    InvalidChar,
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::InvalidChar => "Base64 string contains invalid character",
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::InvalidChar => write!(f, "Base64 string contains invalid character"),
        }
    }
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

/// Convert base64 encoded string to bytes
///
pub fn base64_decode(encoded: &str) -> std::result::Result<Vec<u8>, DecodeError> {
    let decoded_max_len = encoded.len() / 4 * 3 + 3;
    let encoded_data = encoded.as_bytes();
    let mut decoded = Vec::with_capacity(decoded_max_len);

    let mut block3: [u8; 3] = [0, 0, 0];

    for (i, _) in encoded_data.iter().enumerate().step_by(4) {
        let a_ascii = encoded_data[i] as isize - 43;
        let b_ascii = encoded_data[i + 1] as isize - 43;
        let c_ascii = encoded_data[i + 2] as isize - 43;
        let d_ascii = encoded_data[i + 3] as isize - 43;

        if a_ascii < 0
            || a_ascii >= BASE64_REV_LOOKUP.len() as isize
            || b_ascii < 0
            || b_ascii >= BASE64_REV_LOOKUP.len() as isize
            || c_ascii < 0
            || c_ascii >= BASE64_REV_LOOKUP.len() as isize
            || d_ascii < 0
            || d_ascii >= BASE64_REV_LOOKUP.len() as isize
        {
            return Err(DecodeError::InvalidChar);
        }

        //Count the number of '=' padding to remove
        let padding = (a_ascii == 18) as usize
            + (b_ascii == 18) as usize
            + (c_ascii == 18) as usize
            + (d_ascii == 18) as usize;

        let a = BASE64_REV_LOOKUP[a_ascii as usize];
        let b = BASE64_REV_LOOKUP[b_ascii as usize];
        let c = BASE64_REV_LOOKUP[c_ascii as usize];
        let d = BASE64_REV_LOOKUP[d_ascii as usize];

        if a > 63 || b > 63 || c > 63 || d > 63 {
            return Err(DecodeError::InvalidChar);
        }

        block3[0] = ((a << 2) + ((b & 0x30) >> 4)) as u8;
        block3[1] = (((b & 0xf) << 4) + ((c & 0x3c) >> 2)) as u8;
        block3[2] = (((c & 0x3) << 6) + d) as u8;

        decoded.extend(block3.iter().take(3 - padding));
    }

    Ok(decoded)
}

/// Unit tests
///
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

#[test]
fn test_base64_decode_aligned() {
    let decoded_data = base64_decode("QnVpbGQgbWUgdXAgYnV0dGVyY3Vw");
    assert!(decoded_data.is_ok());
    assert_eq!(decoded_data.unwrap(), "Build me up buttercup".as_bytes());
}

#[test]
fn test_base64_decode_unaligned() {
    let decoded_data = base64_decode(
        "V2h5IGRvIHlvdSBidWlsZCBtZSB1cCBidXR0ZXJjdXAgYmFieSBqdXN0IHRvIGxldCBtZSBkb3duPw==",
    );
    assert!(decoded_data.is_ok());
    assert_eq!(
        decoded_data.unwrap(),
        "Why do you build me up buttercup baby just to let me down?".as_bytes()
    );
}

#[test]
fn test_base64_decode_invalid_char() {
    let decoded_data = base64_decode("QnVpbGQgbWU*&gdXAgYnV0dGVyY3Vw");
    assert!(decoded_data.is_err());
    assert_eq!(decoded_data.unwrap_err(), DecodeError::InvalidChar);
}
