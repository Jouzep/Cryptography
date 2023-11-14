use crate::aes::aes_module::*;
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
        ("-aes", "-c") => aes_module(decoded_result, args.remove(4), "cipher"),
        ("-aes", "-d") => aes_module(decoded_result, args.remove(4), "decipher"),
        _ => Vec::new(),
    };
    println!("{}", hex::encode(result));
}

fn convert_little_endian(value: String) -> String {
    let mut char_vec: Vec<char> = value.chars().collect();
    let mut result: Vec<String> = Vec::new();
    let mut tmp: String = String::new();

    // if (value.len() % 2) == 1 {
        // char_vec = prepend(char_vec, '0');
    // }
    for (i, &c) in char_vec.iter().enumerate() {
        tmp.push(c);
        if i % 2 != 0 {
            result.push(tmp.clone());
            tmp.clear();
        }
    }
    result.reverse();
    println!("{:?}", result);
    result.join("")
}

fn gen_key(p: &u64, q: &u64) {
    let result = p * q;
    let totient_n = (p - 1) * (q - 1);
    let mut e = format!("{:x}", 65537);
    let n_hex = format!("{:x}", result);
    println!("{:x}", result);
    let n: String = convert_little_endian(n_hex);
    // e = convert_little_endian(e);

    println!("public key: 0{}-{}", e, n);
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