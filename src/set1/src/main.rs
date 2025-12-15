use anyhow::Result;

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
    let deciphered_input = crack_xor::crack_input(&input, 1);

    println!("{}", String::from_utf8_lossy(&deciphered_input));

    Ok(())
}
