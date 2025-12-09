use anyhow::Result;

mod base64;
mod xor;
mod util;

fn main() -> Result<()> {
    // set 1 - challenge 1
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_input = util::string_to_hex(input)?;
    let encoded_input = base64::encode(&hex_input);

    println!("{}", encoded_input);

    // set 2 - challenge 2
    let input1 = util::string_to_hex("1c0111001f010100061a024b53535009181c")?;
    let input2 = util::string_to_hex("686974207468652062756c6c277320657965")?;
    let xor = xor::xor(&input1, &input2)?;

    println!("{}", util::hex_to_string(&xor));

    Ok(())
}
