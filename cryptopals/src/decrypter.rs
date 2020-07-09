/// Attempts to decrypt the given bytes assumign a single character xor key
/// and that the output is plaintext
///
pub fn single_char_xor_plaintext_decrypt(encrypted: &[u8]) -> Option<Vec<u8>> {
    //Count ascii character occurrences
    let mut counts: [u8; 0xFF] = [0; 0xFF];
    for b in encrypted.iter() {
        counts[*b as usize] += 1;
    }

    //Find the most frequent letter (likely to be one of 'etaoin shrdlu')
    let mut most_common_ascii_encrypted = 0u8;
    let mut max_val = 0u8;
    for (i, &c) in counts.iter().enumerate() {
        if c >= max_val {
            max_val = c;
            most_common_ascii_encrypted = i as u8;
        }
    }

    //XOR of decrypted and encrypted value will give the key
    //We try for all the most common keys and filter out any results that
    //contain non printable characters
    let most_common_ascii_decrypted = b"etaoin shrdlu";
    let mut decrypted_bytes: Vec<u8> = Vec::with_capacity(encrypted.len());
    for c in most_common_ascii_decrypted.iter() {
        let key = most_common_ascii_encrypted ^ c;

        decrypted_bytes.clear();
        decrypted_bytes.extend(encrypted.iter().map_while(|b| decrypt_char(*b, key)));

        if decrypted_bytes.len() == encrypted.len() {
            return Some(decrypted_bytes);
        }
    }

    None
}

/// Perform the xor decryption and return the value if it is printable ascii
///
fn decrypt_char(c: u8, key: u8) -> Option<u8> {
    let d = c ^ key;
    match d {
        32..=126 => Some(d),
        _ => None,
    }
}
