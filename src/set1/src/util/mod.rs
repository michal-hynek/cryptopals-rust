use std::{collections::HashMap, sync::LazyLock};
use thiserror::Error;

static CHAR_VALUES: LazyLock<HashMap<u8, u8>> = LazyLock::<HashMap<u8, u8>>::new(|| HashMap::from([
    (b'0', 0),
    (b'1', 1),
    (b'2', 2),
    (b'3', 3),
    (b'4', 4),
    (b'5', 5),
    (b'6', 6),
    (b'7', 7),
    (b'8', 8),
    (b'9', 9),
    (b'a', 10),
    (b'b', 11),
    (b'c', 12),
    (b'd', 13),
    (b'e', 14),
    (b'f', 15),
    (b'A', 10),
    (b'B', 11),
    (b'C', 12),
    (b'D', 13),
    (b'E', 14),
    (b'F', 15),
]));

#[derive(Error, Debug)]
pub enum HexConversionError {
    #[error("hex string length has to be even")]
    InvalidLength,

    #[error("contains invalid hex digit")]
    InvalidDigit,
}

fn to_hex(byte: &[u8]) -> Result<u8, HexConversionError> {
    let val1 = match CHAR_VALUES.get(&byte[0]) {
        Some(val) => val,
        None => return Err(HexConversionError::InvalidDigit),
    };
    let val2 = match CHAR_VALUES.get(&byte[1]) {
        Some(val) => val,
        None => return Err(HexConversionError::InvalidDigit),
    };

    Ok(val1*16 + val2)
}

// converts hex string input to hex bytes
pub fn string_to_hex(hex_string: &str) -> Result<Vec<u8>, HexConversionError> {
    let mut hex = Vec::new();

    if hex_string.len() % 2 != 0 {
        return Err(HexConversionError::InvalidLength);
    }

    for x in hex_string.as_bytes().chunks_exact(2) {
        hex.push(to_hex(x)?);
    }

    Ok(hex)
}

#[cfg(test)]
mod util_test {
    use super::*;

    #[test]
    fn test_string_to_hex_single_byte() -> Result<(), HexConversionError> {
        assert_eq!(vec![10], string_to_hex("0a")?);
        assert_eq!(vec![11], string_to_hex("0b")?);
        assert_eq!(vec![12], string_to_hex("0c")?);
        assert_eq!(vec![13], string_to_hex("0d")?);
        assert_eq!(vec![14], string_to_hex("0e")?);
        assert_eq!(vec![15], string_to_hex("0f")?);

        Ok(())
    }

    #[test]
    fn test_string_to_hex_single_byte_uppercase_digits() -> Result<(), HexConversionError> {
        assert_eq!(vec![10], string_to_hex("0A")?);
        assert_eq!(vec![11], string_to_hex("0B")?);
        assert_eq!(vec![12], string_to_hex("0C")?);
        assert_eq!(vec![13], string_to_hex("0D")?);
        assert_eq!(vec![14], string_to_hex("0E")?);
        assert_eq!(vec![15], string_to_hex("0F")?);

        Ok(())
    }

    #[test]
    fn test_string_to_hex_multiple_bytes() -> Result<(), HexConversionError> {
        assert_eq!(vec![10, 16], string_to_hex("0A10")?);
        assert_eq!(vec![11, 17], string_to_hex("0B11")?);
        assert_eq!(vec![12, 18], string_to_hex("0C12")?);
        assert_eq!(vec![13, 19], string_to_hex("0D13")?);
        assert_eq!(vec![14, 20], string_to_hex("0E14")?);
        assert_eq!(vec![15, 21], string_to_hex("0F15")?);

        Ok(())
    }
}