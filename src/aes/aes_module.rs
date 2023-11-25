use crate::aes::aes_constant::RCON;
use crate::aes::aes_key_struct::*;
use crate::aes::aes_message::AesMessage;

pub fn gen_expanded_keys(start_key: Key) -> Vec<Key>{
    let mut expanded_keys: Vec<Key> = Vec::new();
    expanded_keys.push(start_key);
    for index in 0..RCON.len() {
        expanded_keys.push(Key::new_w_s_box(expanded_keys.last().unwrap(), index));
    }
    return expanded_keys;
}

pub fn aes_module(message: Vec<u8>, key: String, mode: &str) -> Vec<u8> {
    let expanded_keys = gen_expanded_keys(Key::new(key));
    let mut message = AesMessage::new(message);
    match mode {
        "cipher" => {
            message.cipher(expanded_keys);
        },
        "decipher" => {
            message.decipher(expanded_keys);
        }
        _ => {}
    };
    return Vec::new();
}
