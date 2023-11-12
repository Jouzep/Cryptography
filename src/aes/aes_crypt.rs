// extern crate hex;

// use hex::FromHex;
use crate::aes::aes_key_struct::*;



pub fn aes_crypt(_a: &[u8], key: String) -> Vec<u8> {
    let start_key = Key::new(key);
    let mut expanded_keys: Vec<Key> = Vec::new();
    for index in 0..RCON.len() {
        if let Some(test ) = expanded_keys.last()
        {
            expanded_keys.push(Key::new_w_s_box(test, index));
        } else {
            expanded_keys.push(Key::new_w_s_box(&start_key, index));
        }
    }
    return Vec::new();
}

pub fn aes_decrypt(_a: &[u8], _b: &[u8]) -> Vec<u8> {
    return Vec::new();
}