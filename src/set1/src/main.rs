use anyhow::Result;

mod aes;
mod base64;
mod xor;
mod crack_xor;
mod util;

fn main() -> Result<()> {
    // challenge 1 - convert hex to base64
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_input = util::string_to_hex(input)?;
    let encoded_input = base64::encode(&hex_input);

    println!("{}", encoded_input);

    // challenge 2 - fixed xor
    let input1 = util::string_to_hex("1c0111001f010100061a024b53535009181c")?;
    let input2 = util::string_to_hex("686974207468652062756c6c277320657965")?;
    let xor = xor::xor(&input1, &input2);

    println!("{}", util::hex_to_string(&xor));

    // challenge 3 - single-byte xor cipher
    let input = util::string_to_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;
    let deciphered_input = crack_xor::crack_single_byte_xor(&input);

    println!("{}", String::from_utf8_lossy(&deciphered_input));

    // challenge 4 - detect single-character XOR
    let input = std::fs::read_to_string("input/4.txt")?;
    let input_hex_lines = input
        .split("\n")
        .map(|line| util::string_to_hex(line).unwrap())
        .collect::<Vec<Vec<u8>>>();
    let deciphered_input = crack_xor::crack_single_byte_xors(&input_hex_lines);

    println!("{}", String::from_utf8_lossy(&deciphered_input));

    // challenge 5 - implement repeating-key XOR
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let encrypted_input_hex = xor::xor(input.as_bytes(), "ICE".as_bytes());
    let encrypted_input= util::hex_to_string(&encrypted_input_hex);

    println!("{}", &encrypted_input);

    // challenge 6 - break repeating-key XOR
    let base64_input = std::fs::read_to_string("input/6.txt").unwrap().replace("\n", "");
    let input = base64::decode(base64_input.as_bytes()).unwrap();
    let deciphered_input = crack_xor::crack_multi_byte_xor(&input);

    println!("{}", String::from_utf8_lossy(&deciphered_input));

    // challenge 7 - AES ECB cipher
    let base64_input = std::fs::read_to_string("input/7.txt").unwrap().replace("\n", "");
    let key = "YELLOW SUBMARINE";
    let mut input = base64::decode(base64_input.as_bytes()).unwrap();
    let decrypted_input = aes::decrypt(&mut input, key.as_bytes(), aes::Mode::ECB);

    println!("{}", String::from_utf8_lossy(&decrypted_input));

    Ok(())
}
