use std::collections::HashMap;
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

pub fn crack_input(input: &[u8], _key_len: usize) -> Vec<u8> {
    let key = guess_key(input);
    xor(input, &[key])
}

// returns a single deciphered input
// assumption - only one input in the vector has been xor encrypted
pub fn crack_inputs(inputs: &Vec<Vec<u8>>, key_len: usize) -> Vec<u8> {
    let mut result= Vec::new();
    let mut min_error = 9999999999999999.9;

    for input in inputs {
        let deciphered_input = crack_input(input, key_len);
        let input_error = input_error(&deciphered_input);

        if input_error < min_error {
            result = deciphered_input;
            min_error = input_error;
        }
    }

    result 
}

#[cfg(test)]
mod crack_test {
    use crate::util::HexConversionError;

    use super::*;

    #[test]
    fn test_crack_input() -> Result<(), HexConversionError> {
        let key = 123u8;
        let message = "This is my secret test message.";
        let encrypted_message = xor(message.as_bytes(), &[key]);
        let hex_input = xor(&encrypted_message, &[key]);
        let deciphered_hex = crack_input(&hex_input, 1);
        let deciphered_message = String::from_utf8_lossy(&deciphered_hex);

        assert_eq!(message, deciphered_message);

        Ok(())
    }
}