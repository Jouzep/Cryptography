use crate::aes::aes_message::AesMessage;
use crate::aes::aes_module::gen_expanded_keys;
use crate::aes::aes_key_struct::*;
use crate::rsa::rsa::{crypt_rsa, decrypt_rsa};

use num_bigint::BigUint;
use rand::Rng;

fn generate_symetric_key() -> String {
    let key_value = String::from("1234567890abcdef");
    let mut key = String::new();
    for _ in 0..32 {
        let num = rand::thread_rng().gen_range(0..key_value.len());
        key = format!("{}{}", key, &key_value[num..num + 1]);
    }
    key
}

fn crypt_pgp(message: String, pub_key: &str) {
    let rand_key = generate_symetric_key();
    let aes_key = gen_expanded_keys(Key::new(rand_key.clone()));
    let decode_msg = hex::decode(message).expect("Failed to decode");
    let mut message = AesMessage::new(decode_msg);


    message.cipher(aes_key);
    print!("-");
    crypt_rsa(pub_key,rand_key.to_string(),false);
}

fn decrypt_pgp(message: String, priv_key: &str) {
    let split_message: Vec<&str> = message.split("-").collect();
    let vec_msg = hex::decode(split_message[0]).expect("Failed to decode hexadecimal string");
    let mut crypt_message = AesMessage::new(vec_msg);
    let crypt_key = decrypt_rsa(priv_key,split_message[1]);
    let aes_key = gen_expanded_keys(Key::new(crypt_key));

    crypt_message.decipher(aes_key);
}

pub fn pgp_exec(mode: &str, message: String, rsa_key: &str) {

    match mode {
        "-c" => crypt_pgp(message, rsa_key),
        "-d" => decrypt_pgp(message, rsa_key),
        _ => {}
    }
}