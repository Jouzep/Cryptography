extern crate hex;

use hex::FromHex;

#[derive(Debug)]
struct Key {
    key: String,
    array: Vec<Vec<u32>>,
}

impl Key {
    fn new(key : String) -> Self {
        let mut two_char_array: Vec<String> = Vec::new();
        for chunk in key.chars().collect::<Vec<_>>().chunks(2) {
            let two_chars: String = chunk.iter().collect();
            two_char_array.push(two_chars);
        }
        let decimal_values: Vec<u32> = two_char_array
            .iter()
            .map(|hex_str| u32::from_str_radix(hex_str, 16).unwrap_or_default())
            .collect();
        println!("{:?}", decimal_values);

        let mut decimal_2d_array: Vec<Vec<u32>> = Vec::new();
        for i in 0..4 {
            let start_index = i * 4;
            let end_index = start_index + 4;
            decimal_2d_array.push(decimal_values[start_index..end_index].to_vec());
        }
        Key {
            key,
            array: decimal_2d_array,
        }
    }
}
fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let byte_array_result = Vec::from_hex(hex_string);
    byte_array_result
}

pub fn aes_crypt(a: &[u8], key: String) -> Vec<u8> {
    println!("{:?}", a);
    println!("{}", key);
    // let array_key = hex_string_into_decimal_array(key);
    let array_key = Key::new(key);
    println!("{:?}", array_key);
    return Vec::new();
}

pub fn aes_decrypt(a: &[u8], b: &[u8]) -> Vec<u8> {
    return Vec::new();
}