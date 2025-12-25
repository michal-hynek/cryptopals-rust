mod pkcs7;

fn main() {
    // Challenge 1
    let input = "YELLOW SUBMARINE";
    let mut padded_input = input.as_bytes().to_vec();
    pkcs7::pad(&mut padded_input, 20);

    println!("{:?}", padded_input);
}
