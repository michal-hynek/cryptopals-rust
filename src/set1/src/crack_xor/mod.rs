use core::f64;
use std::{collections::HashMap, ops::Range};
use once_cell::sync::Lazy;

use crate::xor::xor;

static LETTER_FREQUENCIES: Lazy<HashMap<u8, f64>> = Lazy::new(|| {
    let mut frequency_map = HashMap::new();
    let input = std::fs::read_to_string("src/crack_xor/sample_english_input.txt").unwrap();

    for b in input.bytes() {
        frequency_map.entry(b).and_modify(|count| { *count += 1.0 }).or_insert(1.0);
    }

    let chars = frequency_map.keys().copied().collect::<Vec<u8>>();

    for char in chars {
        let freq = frequency_map.get_mut(&char).unwrap();
        *freq /= input.len() as f64;
    }

    frequency_map
});

fn input_error(input: &[u8]) -> f64 {
    let mut char_counts = HashMap::<u8, usize>::new();

    for byte in input.to_ascii_lowercase().iter() {
        char_counts.entry(*byte)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    }

    let mut error = 0.0;
    for (c, _) in LETTER_FREQUENCIES.iter() {
        let expected_count = (input.len() as f64) * LETTER_FREQUENCIES.get(c).unwrap();
        let actual_count = match char_counts.get(c) {
            Some(count) => *count as f64,
            None => 0f64,
        };

        error += (expected_count - actual_count) * (expected_count - actual_count);
    }

    error 
}

fn guess_key(input: &[u8]) -> u8 {
    let mut key = b'a';
    let mut min_error= 999999999999.9;

    for candidate_key in 0u8..255 {
        let xor_input = xor(input, &[candidate_key]);
        let error = input_error(&xor_input);

        if error < min_error {
            min_error = error;
            key = candidate_key;
        }
    }

    key
}

fn hamming_distance(input1: &[u8], input2: &[u8]) -> Result<usize, String> {
    if input1.len() != input2.len() {
        return Err("inputs must have the same length".to_string());
    }

    let distance = input1.iter()
        .zip(input2)
        .map(|(byte1, byte2)| (byte1 ^ byte2).count_ones() as usize)
        .sum();

    Ok(distance)
}

// returns the vector of key size sorted by the probability the given key size is correct
fn guess_key_size(input: &[u8]) -> Vec<usize> {
    let min_key_size: usize = 2;
    let max_key_size = 40;
    let mut guessed_key_sizes = Vec::with_capacity(max_key_size - min_key_size + 1);

    for key_size in min_key_size..=max_key_size {
        if input.len() < key_size*(input.len() / key_size) {
            break;
        }

        let blocks = block_ranges(input, key_size);
        let mut average_distance = 0.0;

        for range in blocks.iter() {
            let block1 = &input[range.0.start..range.0.end];
            let block2 = &input[range.1.start..range.1.end];

            average_distance += hamming_distance(block1, block2).unwrap() as f64 / key_size as f64;
        }

        average_distance /= blocks.len() as f64;
        guessed_key_sizes.push((key_size, average_distance));
    }

    guessed_key_sizes.sort_by(|(_, dist1), (_, dist2)| dist1.total_cmp(dist2));

    guessed_key_sizes.into_iter().map(|(key_size, _)| key_size).collect()
}

fn block_ranges(input: &[u8], key_size: usize) -> Vec<(Range<usize>, Range<usize>)> {
    let n = std::cmp::min(input.len() / key_size, 10000);
    let mut ranges = Vec::new();

    for i in 0..n-1 {
        let from = i*key_size;
        let range1 = Range { start: from, end: from + key_size };
        let range2 = Range { start: range1.end, end: range1.end + key_size };

        ranges.push((range1, range2));
    }

    ranges
}

pub fn crack_single_byte_xor(input: &[u8]) -> Vec<u8> {
    let key = guess_key(input);
    xor(input, &[key])
}

// returns a single deciphered input
// assumption - only one input in the vector has been single-byte xor encrypted
pub fn crack_single_byte_xors(inputs: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut result= Vec::new();
    let mut min_error = f64::MAX;

    for input in inputs {
        let deciphered_input = crack_single_byte_xor(input);
        let input_error = input_error(&deciphered_input);

        if input_error < min_error {
            result = deciphered_input;
            min_error = input_error;
        }
    }

    result 
}

pub fn crack_multi_byte_xor(input: &[u8]) -> Vec<u8> {
    let mut guessed_key = vec![0u8];
    let mut min_error = f64::MAX;

    let key_sizes = guess_key_size(input)[..5].to_vec();

    for key_size in key_sizes {
        let mut key = Vec::new();

        for i in 0..key_size {
            let block_input = input[i..].iter()
                .step_by(key_size)
                .copied()
                .collect::<Vec<u8>>();

            let key_byte = guess_key(&block_input);
            key.push(key_byte);
        }

        let deciphered_input = xor(input, &key);
        let error = input_error(&deciphered_input);

        if error < min_error {
            guessed_key = key;
            min_error = error;
        }
    }

    xor(input, &guessed_key)
}

#[cfg(test)]
mod crack_test {
    use anyhow::Result;

    use crate::util::HexConversionError;

    use super::*;

    #[test]
    fn test_crack_single_byte_xor() -> Result<(), HexConversionError> {
        let key = 123u8;
        let message = "This is my secret test message.";
        let encrypted_message = xor(message.as_bytes(), &[key]);
        let hex_input = xor(&encrypted_message, &[key]);
        let deciphered_hex = crack_single_byte_xor(&hex_input);
        let deciphered_message = String::from_utf8_lossy(&deciphered_hex);

        assert_eq!(message, deciphered_message);

        Ok(())
    }

    #[test]
    fn test_hamming_distance() -> Result<()> {
        let input1 = "this is a test".as_bytes();
        let input2 = "wokka wokka!!!".as_bytes();

        let distance = hamming_distance(input1, input2).unwrap();

        assert_eq!(37, distance);

        Ok(())
    }

    #[test]
    fn test_guess_key_size() {
        let input = std::fs::read_to_string("src/crack_xor/sample_english_input.txt").unwrap();
        let key = "abcdefghijklmnopqrstuvw";
        let enciphred_input = crate::xor::xor(input.as_bytes(), key.as_bytes());

        let guessed_key_size = guess_key_size(&enciphred_input);

        assert!(guessed_key_size[0] == key.len() || guessed_key_size[1] == key.len() || guessed_key_size[2] == key.len());
    }
}