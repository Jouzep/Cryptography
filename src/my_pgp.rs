use crate::aes::aes_module::*;
use crate::rsa::*;
use crate::pgp::*;

fn fill_key(a: &[u8], len: usize) -> Vec<u8> {
    let mut res: Vec<u8> = vec![0; len];
    let mut index = 0;

    for i in 0..len {
        if index == a.len() {
            index = 0;
        }
        res[i] = a[index];
        index += 1;
    }

    res
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();


    let b = if a.len() > b.len() {
        fill_key(b, a.len())
    } else {
        b.to_vec()
    };

    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }

    result
}


fn run_xor_aes(mut args: Vec<String>, content: String) {
    // let key = hex::decode(args[4].to_string()).unwrap();
    let block = if args.contains(&"-b".to_string()) { &args[4] } else { &args[3] };
    let key = hex::decode(block).expect("Failed to decode hexadecimal string");
    let decoded_result = hex::decode(&content).expect("Failed to decode hexadecimal string");
    // let byte_slice: &[u8] = &decoded_result;
    //
    // println!("{:?}", byte_slice);
    let result = match (args[1].as_str(), args[2].as_str()) {
        ("-xor", "-c" | "-d") => xor(&decoded_result, &key),
        ("-aes", "-c") => aes_module(decoded_result, args.remove(4), "cipher", content.len()),
        ("-aes", "-d") => aes_module(decoded_result, args.remove(4), "decipher", content.len()),
        _ => Vec::new(),
    };
    println!("{}", hex::encode(result));
}

pub fn run_pgp(args : Vec<String>, message: String) {
    match args[1].as_str() {
        "-xor" | "-aes" => run_xor_aes(args, message),
        "-rsa" => rsa::run_rsa(args, message),
        "-pgp" => pgp::pgp_exec(args[2].as_str() , message, args[3].as_str()),
        _=>println!("Wrong algo"),
    }
}