pub fn pad(input: &mut Vec<u8>, block_size: usize) {
    if input.len() % block_size == 0 {
        return;
    }

    let pad_len = block_size - (input.len() % block_size);
    let padding = vec![pad_len as u8; pad_len];

    input.extend(padding.iter());
}

#[cfg(test)]
mod test_pkcs7 {
    use super::*;

    #[test]
    fn test_pad_no_extra_chars() {
        let original_input = "abcdefghijkl";
        let mut input = original_input.as_bytes().to_vec();
        let block_size = 4;

        pad(&mut input, block_size);

        assert_eq!(input, original_input.as_bytes());
    }

    #[test]
    fn test_pad_extra_chars() {
        let original_input = "YELLOW SUBMARINE";
        let mut expected_input = "YELLOW SUBMARINE".as_bytes().to_vec();
        expected_input.extend_from_slice(&[0x4, 0x4, 0x4, 0x4]);
        let mut input = original_input.as_bytes().to_vec();
        let block_size = 20;

        pad(&mut input, block_size);

        assert_eq!(input, expected_input);
    }
}