use ecb::cipher::block_padding::Pkcs7;
use ecb::cipher::{BlockDecryptMut, KeyInit};

pub enum Mode {
    ECB,
}

fn decrypt_ecb(input: &mut [u8], key: &[u8]) -> Vec<u8> {
    let plaintext = ecb::Decryptor::<aes::Aes128>::new(key.into())
        .decrypt_padded_mut::<Pkcs7>(input)
        .unwrap();

    plaintext.to_vec()
}

pub fn decrypt(input: &mut [u8], key: &[u8], mode: Mode) -> Vec<u8> {
    match mode {
        Mode::ECB => decrypt_ecb(input, key),
    }
}

#[cfg(test)]
mod test_aes {
    use super::*;

    #[test]
    fn test_decrypt_ecb() {
        let base64_input = std::fs::read_to_string("input/7.txt").unwrap().replace("\n", "");
        let mut input = crate::base64::decode(base64_input.as_bytes()).unwrap();
        let key = "YELLOW SUBMARINE";

        let plaintext = decrypt(&mut input, key.as_bytes(), Mode::ECB);
        let plaintext = String::from_utf8_lossy(&plaintext);

        assert!(plaintext.is_ascii());
    }
}