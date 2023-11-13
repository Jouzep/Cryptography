use std::fmt;
use crate::aes::aes_constant::{RCON};
use crate::aes::aes_function::sub_bytes;

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Key {
    pub array: [[u8; 4]; 4],
}
impl Key {
    pub fn new(key : String) -> Self {
        let mut two_char_array: Vec<String> = Vec::new();
        for chunk in key.chars().collect::<Vec<_>>().chunks(2) {
            let two_chars: String = chunk.iter().collect();
            two_char_array.push(two_chars);
        }
        let decimal_values: Vec<u8> = two_char_array
            .iter()
            .map(|hex_str| u8::from_str_radix(hex_str, 16).unwrap_or_default())
            .collect();

        let double_array: [[u8; 4]; 4] = {
            let mut iter = decimal_values.iter();
            let mut array = [[0u8; 4]; 4];

            for row in array.iter_mut() {
                for elem in row.iter_mut() {
                    *elem = *iter.next().unwrap_or(&0);
                }
            }
            array
        };
        Key {
            array: double_array,
        }
    }

    pub fn new_w_s_box(key : &Key, rcon_index: usize) -> Self {
        let mut test: [[u8; 4]; 4] = [[0; 4]; 4];

        for index1 in 0.. test.len() {
            let w4 = key.array[index1]; // 4 - 4  w-4 for xor
            match index1 {
                0 =>  {
                let mut w1 = key.array[3]; // 4 - 1  w-1
                w1.rotate_left(1); // Rotate
                for (index2, element2) in w1.iter_mut().enumerate() {
                    if index2 == 0 {
                        *element2 = sub_bytes(element2) ^ w4[index2] ^ RCON[rcon_index];
                    }
                    else {
                        *element2 = sub_bytes(element2) ^ w4[index2];
                    }
                }
                test[index1] = w1;
            } ,
            _ => {
                let mut w1 = test[index1 - 1]; // 4 - 1  w-1
                for (index2, element2) in w1.iter_mut().enumerate() {
                    *element2 = *element2 ^ w4[index2];
                }
                test[index1] = w1;
            }
        }
            }
        Key {
            array: test
        }
    }
}

