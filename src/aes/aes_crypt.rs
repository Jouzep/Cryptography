// extern crate hex;

// use hex::FromHex;
use crate::aes::aes_key_struct::*;
use crate::aes::aes_message::AesMessage;

pub fn gen_expanded_keys(start_key: Key) -> Vec<Key>{
    let mut expanded_keys: Vec<Key> = Vec::new();
    expanded_keys.push(start_key);
    for index in 0..RCON.len() {
        expanded_keys.push(Key::new_w_s_box(expanded_keys.last().unwrap(), index));
    }
    for (index, item) in expanded_keys.iter().enumerate() {
        println!("Key {}: {}",index ,item);
    }
    return expanded_keys;
}

pub fn subbytes_message(mut msg: Vec<u8>) -> Vec<u8> {
    println!("{:?}", msg);
    for element in &mut msg {
        *element = sub_bytes(element);
    }
    println!("{:?}", msg);
    return msg;
}
pub fn aes_crypt(message: Vec<u8>, key: String) -> Vec<u8> {
    let start_key = Key::new(key);
    let expanded_keys = gen_expanded_keys(start_key);
    let mut message = AesMessage::new(message);
    println!("{:?}", message.array);
    message.sub_bytes();
    println!("{:?}",message.array);

    message.shift_rows();
    return Vec::new();
}

pub fn aes_decrypt(_a: &[u8], _b: &[u8]) -> Vec<u8> {
    return Vec::new();
}