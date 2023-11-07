use std::ops::BitXor;

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![];

    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }
    result
}

fn aes_crypt(a: &[u8], b: &[u8]) -> Vec<u8> {
    return Vec::new();
}

fn aes_decrypt(a: &[u8], b: &[u8]) -> Vec<u8> {
    return Vec::new();
}

fn run_xor_aes(args: &[String], content: String) {
    let key = hex::decode(args[4].to_string()).unwrap();
    let message = hex::decode(content).unwrap();
    let result = match (args[1].as_str(), args[2].as_str()) {
        ("-xor", "-c" | "-d") => (xor(&message, &key)),
        ("-aes", "-c") => aes_crypt(&message, &key),
        ("-aes", "-d") => aes_decrypt(&message, &key),
        _ => Vec::new(),
    };
    println!("{}", hex::encode(result));
}

fn run_rsa() {

}

pub fn run_pgp(args: &[String], message: String) {
    match args[1].as_str() {
        "-xor" | "-aes" => run_xor_aes(args, message),
        "-rsa"=> run_rsa(),
        _=>println!("Wrong algo"),
    }
}