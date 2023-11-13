use crate::aes::aes_crypt::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![];

    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }
    result
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
        ("-aes", "-c") => aes_crypt(decoded_result, args.remove(4)),
        ("-aes", "-d") => aes_decrypt(&decoded_result, &key),
        _ => Vec::new(),
    };
    println!("{}", hex::encode(result));
}

fn convert_little_endian(value: String) {
    let char_vec: Vec<char> = value.chars().collect();
    let result: Vec<char>;
    for i in char_vec {
        println!("{}", i);
    }
}

fn gen_key(p: &u64, q: &u64) {
    let result = p * q;
    let totient_n = (p - 1) * (q - 1);
    let e = 65537;
    let n_hex = format!("{:x}", result);
    println!("{:x}", result);
    convert_little_endian(n_hex);

    // println!("public key: {:x}-{:x}", e, result);
}

fn run_rsa(args: Vec<String>, message: String) {
    let p =u64::from_str_radix(&args[3], 16).expect("Failed to decode hexadecimal string");
    let q =u64::from_str_radix(&args[4], 16).expect("Failed to decode hexadecimal string");
    let result = match args[2].as_str() {
        "-g" => gen_key(&p, &q),
        _ => println!("Wrong rsa flag"),
    };
}

pub fn run_pgp(args : Vec<String>, message: String) {
    match args[1].as_str() {
        "-xor" | "-aes" => run_xor_aes(args, message),
        "-rsa"=> run_rsa(args, message),
        _=>println!("Wrong algo"),
    }
}