use crate::base64_converter::base64_decode;
use crate::hamming::bitwise_hamming_dist;
use crate::xor_decrypter::single_char_xor_plaintext_decrypt;
use std::fs;

/// Break a repeating key XOR by gusessing the key size and then transposing
/// the data to perform single character xor breaking on the columns
///
pub fn solve() -> String {
    let encoded = fs::read_to_string("payloads/set1/challenge6.txt").unwrap();
    let decoded = base64_decode(&encoded).unwrap();

    //Find the likely keysize (puzzle says between 2 and 40)
    let keysize = find_keysize(&decoded, 2, 40, 6);

    //Transpose the data into columns and decrypt each colum using single xor char
    //the combination of each single char forms the overall repeating xor key
    let transposed = transpose(&decoded, keysize);

    let key = transposed
        .chunks(keysize)
        .take(keysize)
        .map(|chunk| single_char_xor_plaintext_decrypt(chunk).unwrap().2)
        .collect::<Vec<u8>>();

    let decrypted = decoded
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % keysize])
        .collect();

    String::from_utf8(decrypted).unwrap()
}

/// Finds the key size that is likely to have been used to xor encrypt the data
///
fn find_keysize(
    data: &[u8],
    min_keysize: usize,
    max_keysize: usize,
    av_over_num_pairs: usize,
) -> usize {
    //Take first two chunks of potential keysize
    //Calculate the hamming distance
    //Smallest normalised hamming distance is prob the key size
    let mut smallest_normalised_dist = f32::MAX;
    let mut likely_keysize = 0usize;

    for keysize in min_keysize..=max_keysize {
        let mut chunks_it = data.chunks(keysize);

        let total_dist = (0..av_over_num_pairs).fold(0, |acc, _| {
            acc + bitwise_hamming_dist(chunks_it.next().unwrap(), chunks_it.next().unwrap())
                .unwrap()
        });

        let av_dist = total_dist as f32 / 3.0;

        let normalised = av_dist / keysize as f32;
        if normalised <= smallest_normalised_dist {
            smallest_normalised_dist = normalised;
            likely_keysize = keysize;
        }
    }

    likely_keysize
}

/// Transposes a matrix to a square matrix such that abc def ghi jkl => aei bfj cgk dhl
///
fn transpose(matrix: &[u8], stride: usize) -> Vec<u8> {
    let mut t = vec![0; stride * stride];
    for i in 0..t.len() {
        let x = i / stride;
        let y = i % stride;
        let t_idx = y * stride + x;
        t[t_idx] = matrix[i];
    }
    t
}

/// Unit tests
///
#[test]
fn test_transpose() {
    let m = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let t = transpose(&m, 3);
    assert_eq!(t, [0, 3, 6, 1, 4, 7, 2, 5, 8]);
}
