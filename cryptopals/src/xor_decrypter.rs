static PRINTABLE_ASCII_SCORES: [u32; 256] = init_scores_table();

/// Attempts to decrypt the given bytes assumign a single character xor key
/// and that the output is plaintext.
///
pub fn single_char_xor_plaintext_decrypt(encrypted: &[u8]) -> Option<(Vec<u8>, u32, u8)> {
    //Count ascii character occurrences
    let mut counts: [u8; 256] = [0; 256];
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
    //We try for all the most common keys and filter out any results that contain non printable characters
    //we score based on letter frequencies and return the one with the highest score
    let mut decrypted_bytes: Vec<u8> = Vec::with_capacity(encrypted.len());
    let mut decrypted_bytes_best_score: Vec<u8> = Vec::with_capacity(encrypted.len());
    let mut best_score = 0;
    let mut best_key = 0;

    for c in 32..=126 {
        let key = most_common_ascii_encrypted ^ c;

        decrypted_bytes.clear();
        decrypted_bytes.extend(encrypted.iter().map_while(|b| try_decrypt_char(*b, key)));

        if decrypted_bytes.len() == encrypted.len() {
            let score = freq_score(&decrypted_bytes);
            if score >= best_score {
                best_score = score;
                best_key = key;
                decrypted_bytes_best_score.clear();
                decrypted_bytes_best_score.extend(&decrypted_bytes);
            }
        }
    }

    if !decrypted_bytes_best_score.is_empty() {
        Some((decrypted_bytes_best_score, best_score, best_key))
    } else {
        None
    }
}

/// Perform the xor decryption and return the value if it is printable ascii
///
fn try_decrypt_char(c: u8, key: u8) -> Option<u8> {
    let d = c ^ key;
    match d {
        32..=126 | 10 => Some(d), //Some of the valid strings have new lines in them
        _ => None,
    }
}

/// Score the potential plaintext based on letter frequencies, strings with a high score are more likely to be English text
///
fn freq_score(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .map(|b| PRINTABLE_ASCII_SCORES[*b as usize])
        .sum()
}

/// Initialise the printable ASCII scores based on letter frequencies
/// Converted from float to int for easier comparisons
///
const fn init_scores_table() -> [u32; 256] {
    let mut lut: [u32; 256] = [0; 256];
    lut[b' ' as usize] = 1918182;
    lut[b'a' as usize] = 651738;
    lut[b'b' as usize] = 124248;
    lut[b'c' as usize] = 217339;
    lut[b'd' as usize] = 349835;
    lut[b'e' as usize] = 1041442;
    lut[b'f' as usize] = 197881;
    lut[b'g' as usize] = 158610;
    lut[b'h' as usize] = 492888;
    lut[b'i' as usize] = 558094;
    lut[b'j' as usize] = 9033;
    lut[b'k' as usize] = 50529;
    lut[b'l' as usize] = 331490;
    lut[b'm' as usize] = 202124;
    lut[b'n' as usize] = 564513;
    lut[b'o' as usize] = 596302;
    lut[b'p' as usize] = 137645;
    lut[b'q' as usize] = 8606;
    lut[b'r' as usize] = 497563;
    lut[b's' as usize] = 515760;
    lut[b't' as usize] = 729357;
    lut[b'u' as usize] = 225134;
    lut[b'v' as usize] = 82903;
    lut[b'w' as usize] = 171272;
    lut[b'x' as usize] = 13692;
    lut[b'y' as usize] = 145984;
    lut[b'z' as usize] = 7836;
    lut[b'A' as usize] = 651738;
    lut[b'B' as usize] = 124248;
    lut[b'C' as usize] = 217339;
    lut[b'D' as usize] = 349835;
    lut[b'E' as usize] = 1041442;
    lut[b'F' as usize] = 197881;
    lut[b'G' as usize] = 158610;
    lut[b'H' as usize] = 492888;
    lut[b'I' as usize] = 558094;
    lut[b'J' as usize] = 9033;
    lut[b'K' as usize] = 50529;
    lut[b'L' as usize] = 331490;
    lut[b'M' as usize] = 202124;
    lut[b'N' as usize] = 564513;
    lut[b'O' as usize] = 596302;
    lut[b'P' as usize] = 137645;
    lut[b'Q' as usize] = 8606;
    lut[b'R' as usize] = 497563;
    lut[b'S' as usize] = 515760;
    lut[b'T' as usize] = 729357;
    lut[b'U' as usize] = 225134;
    lut[b'V' as usize] = 82903;
    lut[b'W' as usize] = 171272;
    lut[b'X' as usize] = 13692;
    lut[b'Y' as usize] = 145984;
    lut[b'Z' as usize] = 7836;
    lut
}
