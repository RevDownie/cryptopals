use crate::hex_converter::hex_decode;
use std::fs::File;
use std::io::{self, BufRead};

/// Look for the ciphertext in the hex encoded file that has the most repeating 16 byte blocks
/// as this is likely to mean it has been encoded with a 16 byte ECB key.
///
pub fn solve() -> String {
    let payload_file = File::open("payloads/set1/challenge8.txt").unwrap();
    let encoded_lines: Vec<String> = io::BufReader::new(payload_file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let decoded_lines = encoded_lines.iter().map(|line| hex_decode(&line).unwrap());

    //Convert the 16 bytes into a 128 bit int for quicker comparison and sort them so
    //we can count the number of matching pairs just by counting the run lengths
    let line_block_ids: Vec<Vec<u128>> = decoded_lines.map(|line| pack_128_chunks(&line)).collect();
    let num_repeating_blocks = line_block_ids.iter().map(|ids| count_runs(&*ids));

    //The line with the most repeating blocks is likely ECB encrypted
    let (idx, _) = num_repeating_blocks
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .unwrap();

    encoded_lines[idx].clone()
}

/// Takes 16 byte blocks and packs them into 128 bit ints ans sorts for quicker comparisons
///
fn pack_128_chunks(bytes: &[u8]) -> Vec<u128> {
    let mut chunks = Vec::with_capacity(bytes.len() % 16);
    for i in (0..bytes.len()).step_by(16) {
        let p = ((bytes[i] as u128) << 120)
            | ((bytes[i + 1] as u128) << 112)
            | ((bytes[i + 2] as u128) << 104)
            | ((bytes[i + 3] as u128) << 96)
            | ((bytes[i + 4] as u128) << 88)
            | ((bytes[i + 5] as u128) << 80)
            | ((bytes[i + 6] as u128) << 72)
            | ((bytes[i + 7] as u128) << 64)
            | ((bytes[i + 8] as u128) << 56)
            | ((bytes[i + 9] as u128) << 48)
            | ((bytes[i + 10] as u128) << 40)
            | ((bytes[i + 11] as u128) << 32)
            | ((bytes[i + 12] as u128) << 24)
            | ((bytes[i + 13] as u128) << 16)
            | ((bytes[i + 14] as u128) << 8)
            | (bytes[i + 15] as u128);
        chunks.push(p);
    }

    chunks.sort_unstable();
    chunks
}

/// Counts the number of matches in a sorted list
///
fn count_runs(bytes: &[u128]) -> usize {
    let mut run_count = 0;
    let mut prev = bytes[0];
    for &b in bytes[1..].iter() {
        if b == prev {
            run_count += 1;
            prev = b;
        }
    }
    run_count
}
