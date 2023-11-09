use std::ops::BitXor;
use hex::FromHex;

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![];

    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }
    result
}
fn hex_string_into_octal_array(hex_string: String) -> Vec<String> {
    let mut two_char_array: Vec<String> = Vec::new();

    for chunk in hex_string.chars().collect::<Vec<_>>().chunks(2) {
        let two_chars: String = chunk.iter().collect();
        two_char_array.push(two_chars);
    }
    return two_char_array;
}
fn aes_crypt(a: &[u8], key: String) -> Vec<u8> {
    println!("{:?}", a);
    println!("{}", key);
    let array_key = hex_string_into_octal_array(key);
    println!("{:?}", array_key);
    return Vec::new();
}

fn aes_decrypt(a: &[u8], b: &[u8]) -> Vec<u8> {
    return Vec::new();
}

fn run_xor_aes(mut args: Vec<String>, content: String) {
    // let key = hex::decode(args[4].to_string()).unwrap();
    let key = hex::decode(&args[4]).expect("Failed to decode hexadecimal string");
    let decoded_result = hex::decode(content).expect("Failed to decode hexadecimal string");
    // let byte_slice: &[u8] = &decoded_result;
    //
    // println!("{:?}", byte_slice);
    let result = match (args[1].as_str(), args[2].as_str()) {
        ("-xor", "-c" | "-d") => xor(&decoded_result, &key),
        ("-aes", "-c") => aes_crypt(&decoded_result, args.remove(4)),
        ("-aes", "-d") => aes_decrypt(&decoded_result, &key),
        _ => Vec::new(),
    };
    println!("{}", hex::encode(result));
}

    fn run_rsa() {

}

pub fn run_pgp(args : Vec<String>, message: String) {
    match args[1].as_str() {
        "-xor" | "-aes" => run_xor_aes(args, message),
        "-rsa"=> run_rsa(),
        _=>println!("Wrong algo"),
    }
}