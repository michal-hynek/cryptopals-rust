use std::{collections::HashMap, sync::LazyLock};

use bitvec::prelude::*;
use once_cell::sync::Lazy;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Base64Error {
    #[error("invalid base64 character {character:?}")]
    InvalidCharacter {
        character: u8,
    },

    #[error("invalid base64 input length")]
    InvalidLength,
}

static BASE64_MAPPING: LazyLock<HashMap<u8, u8>> = LazyLock::new(|| HashMap::from([
    (0, b'A'), (1, b'B'), (2, b'C'), (3, b'D'), (4, b'E'), (5, b'F'), (6, b'G'),
    (7, b'H'), (8, b'I'), (9, b'J'), (10, b'K'), (11, b'L'), (12, b'M'), (13, b'N'),
    (14, b'O'), (15, b'P'), (16, b'Q'), (17, b'R'), (18, b'S'), (19, b'T'), (20, b'U'),
    (21, b'V'), (22, b'W'), (23, b'X'), (24, b'Y'), (25, b'Z'), (26, b'a'), (27, b'b'),
    (28, b'c'), (29, b'd'), (30, b'e'), (31, b'f'), (32, b'g'), (33, b'h'), (34, b'i'),
    (35, b'j'), (36, b'k'), (37, b'l'), (38, b'm'), (39, b'n'), (40, b'o'), (41, b'p'),
    (42, b'q'), (43, b'r'), (44, b's'), (45, b't'), (46, b'u'), (47, b'v'), (48, b'w'),
    (49, b'x'), (50, b'y'), (51, b'z'), (52, b'0'), (53, b'1'), (54, b'2'), (55, b'3'),
    (56, b'4'), (57, b'5'), (58, b'6'), (59, b'7'), (60, b'8'), (61, b'9'), (62, b'+'),
    (63, b'/'),
]));

static BASE64_REVERSE_MAPPING: Lazy<HashMap<u8, u8>> = Lazy::new(|| {
    let mut mapping = HashMap::new();

    for (code, char) in BASE64_MAPPING.iter() {
        mapping.insert(*char, *code);
    }

    mapping
});

fn pad(input: &[u8], output: &mut Vec<u8>) {
    if input.len() % 3 == 1 {
        output.push(b'=');
        output.push(b'=');
    }

    if input.len() % 3 == 2 {
        output.push(b'=');
    }
}

pub fn encode(input: &[u8]) -> String {
    let bits= BitVec::<u8, Msb0>
        ::from_vec(input.to_vec());
    let chunks = bits.chunks(6);

    let mut output = Vec::new();
    for chunk in chunks {
        let mut val: u8 = chunk.iter().fold(0, |acc, x| {
            if *x {
                acc * 2 + 1
            } else {
                acc * 2
            }
        });

        // pad to the chunk length of 6
        for _ in chunk.len()..6 {
            val *= 2;
        }

        output.push(*BASE64_MAPPING.get(&val).unwrap());
    }

    pad(input, &mut output);

    String::from_utf8_lossy(&output).into()
}

pub fn decode(base64_input: &[u8]) -> Result<Vec<u8>, Base64Error> {
    if base64_input.len() * 6 % 8 != 0 {
        return Err(Base64Error::InvalidLength);
    }

    let mut decoded_bits = BitVec::<u8, Msb0>::new();
    let mut contains_padding = false;

    for base64_char in base64_input {
        if *base64_char == b'=' {
            contains_padding = true;
            break;
        }

        let base64_val = match BASE64_REVERSE_MAPPING.get(base64_char) {
            Some(val) => val,
            None => return Err(Base64Error::InvalidCharacter { character: *base64_char }),
        };

        decoded_bits.push((*base64_val & (1 << 5)) != 0);
        decoded_bits.push((*base64_val & (1 << 4)) != 0);
        decoded_bits.push((*base64_val & (1 << 3)) != 0);
        decoded_bits.push((*base64_val & (1 << 2)) != 0);
        decoded_bits.push((*base64_val & (1 << 1)) != 0);
        decoded_bits.push((*base64_val & 1) != 0);
    }

    // remove the trailing 0 bits added when base64 padded to the byte boundary
    if contains_padding {
        while decoded_bits.len() % 8 != 0 {
            decoded_bits.pop();
        }
    }

    Ok(decoded_bits.into())
}

#[cfg(test)]
mod base64_test {
    use rand::{Rng, rngs::ThreadRng};

    use super::*;

    #[test]
    fn test_encode_empty_bytes() {
        assert_eq!("", encode(&Vec::new()));
    }

    #[test]
    fn test_encode_no_padding() {
        let input = "Many hands make light work.".as_bytes();
        let expected_base64 = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";

        assert_eq!(expected_base64, encode(input));
    }

    #[test]
    fn test_encode_single_padding() {
        let input = "Hello World".as_bytes();
        let expected_base64 = "SGVsbG8gV29ybGQ=";

        assert_eq!(expected_base64, encode(input));
    }

    #[test]
    fn test_encode_double_padding() {
        let input = "Hola Mundo".as_bytes();
        let expected_base64 = "SG9sYSBNdW5kbw==";

        assert_eq!(expected_base64, encode(input));
    }

    #[test]
    fn test_encode_cryptopals_test_case() {
        let input: Vec<u8> = vec![
            0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e,
            0x67, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x62, 0x72, 0x61,
            0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20,
            0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20,
            0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d,
        ];
        let expected_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(expected_base64, encode(&input));
    }

    #[test]
    fn test_decode_empty_base64() {
        let decoded = decode(&Vec::new()).unwrap();

        assert!(decoded.is_empty()); 
    }

    #[test]
    fn test_decode_error_for_invalid_length() {
        let base64 = "YW";

        let decode_result = decode(base64.as_bytes());

        assert!(decode_result.is_err());
        assert_eq!(Base64Error::InvalidLength, decode_result.err().unwrap());
    }

    #[test]
    fn test_decode_error_for_invalid_character() {
        let base64 = "YWJ$";

        let decode_result = decode(base64.as_bytes());

        assert!(decode_result.is_err());
        assert_eq!(Base64Error::InvalidCharacter { character: b'$' }, decode_result.err().unwrap());
    }

    #[test]
    fn test_decode_no_padding() {
        let base64 = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";
        let expected_decoded= "Many hands make light work.".as_bytes();

        let decoded = decode(base64.as_bytes()).unwrap();

        assert_eq!(expected_decoded, decoded);
    }

    #[test]
    fn test_decode_single_padding() {
        let base64 = "SGVsbG8gV29ybGQ=";
        let expected_decoded = "Hello World".as_bytes();

        let decoded = decode(base64.as_bytes()).unwrap();

        assert_eq!(expected_decoded, decoded);
    }

    #[test]
    fn test_decode_double_padding() {
        let base64 = "SG9sYSBNdW5kbw==";
        let expected_decoded = "Hola Mundo".as_bytes();

        let decoded = decode(base64.as_bytes()).unwrap();

        assert_eq!(expected_decoded, decoded);
    }

    #[test]
    fn test_encode_decode_fuzz() {
        let mut rng = rand::rng();

        for _ in 0..1000 {
            let input = generate_random_input(&mut rng);
            let decoded = decode(encode(&input).as_bytes()).unwrap();

            assert_eq!(input, decoded);
        }
    }

    fn generate_random_input(rng: &mut ThreadRng) -> Vec<u8> {
        let len = rng.random_range(0..=100);
        let mut input = Vec::new();

        for _ in 0..len {
            let rand_byte = rng.random_range(0..255) as u8;
            input.push(rand_byte);
        }

        input
    }
}