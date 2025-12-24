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

pub fn is_ecb_encrypted(input: &[u8]) -> bool {
    let block_size = 16;

    for start1 in (0..input.len()-block_size).step_by(block_size) {
        for start2 in (start1+block_size..input.len()).step_by(block_size) {
            if input[start1..start1+block_size] == input[start2..start2+block_size] {
                return true;
            }
        }
    }

    false
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

    #[test]
    fn test_ecb_encrypted_true_for_ecb() {
        let input = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        let hex_input = crate::util::string_to_hex(&input).unwrap();

        assert!(is_ecb_encrypted(&hex_input));
    }

    #[test]
    fn test_ecb_encrypted_false_for_not_ecb() {
        let input = "9eed35024a40add6409a9690e570ef357dfc0b38491706783dbb6043bd4fcdaf01986fcccbf89f15bc53fe4aff70821b309aa5cec59ef3c588c1042593f9994644bca862152a20bf94dc0d288176eb9f49b7f814bf35050e83b139d2dbd5f08d3cef35e271ccc6d8074fc5fe1570886a0746ce19be8cea27c4382bd04d8d45c7b7fd9e3e89ad38eb37656577395fa0062e5f8e15be2c9a4833bb1f2fce90bb86";
        let hex_input = crate::util::string_to_hex(&input).unwrap();

        assert!(!is_ecb_encrypted(&hex_input));
    }
}