// extern crate hex;

// use hex::FromHex;
use crate::aes::aes_key_struct::*;



pub fn aes_crypt(_a: &[u8], key: String) -> Vec<u8> {
    let start_key = Key::new(key);

    let _second_key = Key::new_w_s_box(&start_key);
    return Vec::new();
}

pub fn aes_decrypt(_a: &[u8], _b: &[u8]) -> Vec<u8> {
    return Vec::new();
}