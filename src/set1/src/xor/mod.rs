use bitvec::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum XorError {
    #[error("input lengths must be equal")]
    InvalidLengths,
}

pub fn xor(x: &[u8], y: &[u8]) -> Result<Vec<u8>, XorError> {
    if x.len() != y.len() {
        return Err(XorError::InvalidLengths);
    }

    let x_bits = BitVec::<u8, Msb0>::from_vec(x.to_vec());
    let y_bits = BitVec::<u8, Msb0>::from_vec(y.to_vec());
    let mut x_xor_y = BitVec::<u8, Msb0>::from_vec(vec![0; x.len()]);

    for (i, (x, y)) in x_bits.iter().zip(y_bits.iter()).enumerate() {
        if *x == *y {
            x_xor_y.set(i, false);
        } else {
            x_xor_y.set(i, true);
        }
    }

    Ok(x_xor_y.into())
}

#[cfg(test)]
mod test_xor {
    use super::*;

    #[test]
    fn test_xor_error_for_different_length_inputs() {
        let result = xor(&[0x1, 0x2], &[0x1]);

        assert!(result.is_err());
    }

    #[test]
    fn test_xor_single_byte() -> Result<(), XorError> {
        assert_eq!(vec![0x0], xor(&[0x0], &[0x0])?);
        assert_eq!(vec![0x1], xor(&[0x1], &[0x0])?);
        assert_eq!(vec![0x1], xor(&[0x0], &[0x1])?);
        assert_eq!(vec![0x0], xor(&[0x1], &[0x1])?);

        Ok(())
    }

    #[test]
    fn test_xor_multiple_bytes() -> Result<(), XorError> {
        let x = [
            0x1c, 0x01, 0x11, 0x00, 0x1f, 0x01, 0x01,
            0x00, 0x06, 0x1a, 0x02, 0x4b, 0x53, 0x53,
            0x50, 0x09, 0x18, 0x1c
        ];
        let y = [
            0x68, 0x69, 0x74, 0x20, 0x74, 0x68, 0x65,
            0x20, 0x62, 0x75, 0x6c, 0x6c, 0x27, 0x73,
            0x20, 0x65, 0x79, 0x65
        ];
        let x_xor_y = vec![
            0x74, 0x68, 0x65, 0x20, 0x6b, 0x69, 0x64,
            0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20,
            0x70, 0x6c, 0x61, 0x79
        ];

        let result = xor(&x, &y)?;

        assert_eq!(x_xor_y, result);

        Ok(())
    }
}