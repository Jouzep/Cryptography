use crate::aes::aes_function::{inv_shift_row_single, inv_sub_bytes, shift_row_single, xor_matrix, sub_bytes};
use crate::aes::aes_key_struct::{Key};

#[derive(Debug)]
pub struct AesMessage {
    pub array: Vec<[u8; 4]>,
}
fn mix_columns(column: &mut [u8; 4]) {
    let mut result_column = [0; 4];
    for i in 0..4 {
        result_column[i] = galois_multiply(0x02, column[i]) ^ galois_multiply(0x03, column[(i + 1) % 4])
            ^ galois_multiply(0x01, column[(i + 2) % 4]) ^ galois_multiply(0x01, column[(i + 3) % 4]);
    }
    column.copy_from_slice(&result_column);
}

fn galois_multiply(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;

    for _ in 0..8 {
        if b & 1 != 0 {
            result ^= a;
        }
        let high_bit_set = (a & 0x80) != 0;
        a <<= 1;
        if high_bit_set {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    result
}

fn inv_mix_columns(column: &mut [u8; 4]) {
    let mut result_column = [0; 4];
    for i in 0..4 {
        result_column[i] = galois_multiply(0x0e, column[i])
            ^ galois_multiply(0x0b, column[(i + 1) % 4])
            ^ galois_multiply(0x0d, column[(i + 2) % 4])
            ^ galois_multiply(0x09, column[(i + 3) % 4]);
    }
    column.copy_from_slice(&result_column);
}

impl AesMessage {
    pub fn new(message: Vec<u8>) -> Self {
        let mut double_array: Vec<[u8; 4]> = {
            let double_array: Vec<_> = message.chunks(4).map(|chunk| {
                let mut padded_chunk = [0; 4];
                padded_chunk[..chunk.len()].copy_from_slice(chunk);
                padded_chunk
            }).collect();
            double_array
        };
        let modulos =  (double_array.len()) % 4 ;
        if modulos == 0 {
            return AesMessage {
                array: double_array
            }
        }
        for _ in 0..4 - modulos {
            double_array.push([0; 4]);
        }
        AesMessage {
            array: double_array
        }
    }

    pub fn sub_bytes(&mut self, mode: &str) {
        match mode {
            "cipher" => {
                for element in self.array.iter_mut() {
                    for sub_element in element.iter_mut() {
                        *sub_element = sub_bytes(&sub_element);
                    }
                }
            },
            "decipher" => {
                for element in self.array.iter_mut() {
                    for sub_element in element.iter_mut() {
                        *sub_element = inv_sub_bytes(&sub_element);
                    }
                }
            }

            _ => {}
        }
    }
    pub fn shift_rows(&mut self, mode: &str) {
        match mode {
            "cipher" => {
                for chunk in self.array.chunks_mut(4) {
                    shift_row_single(chunk, 1);
                    shift_row_single(chunk, 2);
                    shift_row_single(chunk, 3);
                }
            },
            "decipher" => {
                for chunk in self.array.chunks_mut(4) {
                    inv_shift_row_single(chunk, 1);
                    inv_shift_row_single(chunk, 2);
                    inv_shift_row_single(chunk, 3);
                }
            }
            _ => {}
        }

    }
    pub fn mix_columns(&mut self, mode : &str) {
        match mode {
            "cipher" => {
                for element in self.array.iter_mut() {
                    mix_columns(element);
                }
            },
            "decipher" => {
                for element in self.array.iter_mut() {
                    inv_mix_columns(element);
                }
            }
            _ => {}
        }

    }
    pub fn add_round_key(&mut self, key: &Key) {
        for element in self.array.chunks_mut(4) {
            xor_matrix(element, &key.array);
        }
    }

    pub fn cipher(&mut self, expanded_keys : Vec<Key>) {
        self.add_round_key(&expanded_keys[0]);
        for i in 1..expanded_keys.len()-1 {
            self.sub_bytes("cipher");
            self.shift_rows("cipher");
            self.mix_columns("cipher");
            self.add_round_key(&expanded_keys[i]);
        }
        self.sub_bytes("cipher");
        self.shift_rows("cipher");
        self.add_round_key(&expanded_keys.last().unwrap());
        for row in &self.array {
            for &element in row {
                print!("{:02x}", element);
            }
        }
    }

    pub fn decipher(&mut self, expanded_keys : Vec<Key>) {
        self.add_round_key(&expanded_keys[expanded_keys.len() - 1]);
        for i in (1..expanded_keys.len()-1).rev() {
            self.shift_rows("decipher");
            self.sub_bytes("decipher");
            self.add_round_key(&expanded_keys[i]);
            self.mix_columns("decipher");
        }
        self.shift_rows("decipher");
        self.sub_bytes("decipher");
        self.add_round_key(&expanded_keys[0]);
        for row in &self.array {
            for &element in row {
                print!("{:02x}", element);
            }
        }
    }
}

