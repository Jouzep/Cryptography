use crate::aes::aes_module::gen_expanded_keys;
use crate::aes::aes_key_struct::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_random_hex_string(length: usize) -> String {
    let mut rng = thread_rng();

    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    random_string
}

pub fn pgp_exec(mut args: Vec<String>, message: String) {
    let rand_key = generate_random_hex_string(16);
    let aes_key = gen_expanded_keys(Key::new(rand_key));

    println!("{:?}", aes_key);
}