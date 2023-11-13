use crate::aes::aes_key_struct::{sub_bytes, Key};

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
    // Copy the result back to the original column
    column.copy_from_slice(&result_column);
}

fn galois_multiply(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;
    let mut carry = 0;

    for _ in 0..8 {
        if b & 1 != 0 {
            result ^= a;
        }
        let high_bit_set = (a & 0x80) != 0;
        a <<= 1;
        if high_bit_set {
            a ^= 0x1b; // XOR with the irreducible polynomial x^8 + x^4 + x^3 + x + 1
        }
        b >>= 1;
    }
    result
}

impl AesMessage {
    pub fn new(mut message: Vec<u8>) -> Self {
        let mut double_array: Vec<[u8; 4]> = {
            let double_array: Vec<_> = message.chunks(4).map(|chunk| {
                let mut padded_chunk = [0; 4];
                padded_chunk[..chunk.len()].copy_from_slice(chunk);
                padded_chunk
            }).collect();
            double_array
        };
        let modulos =  double_array.len() % 4;
        for i in 0..modulos {
            double_array.push([0; 4]);
        }
        AesMessage {
            array: double_array
        }
    }

    pub fn sub_bytes(&mut self) {
        for element in self.array.iter_mut() {
            for sub_element in element.iter_mut() {
                *sub_element = sub_bytes(&sub_element);
            }
        }
    }
    pub fn shift_rows(&mut self) {
        for (index, chunk) in self.array.chunks_mut(4).enumerate() {
            shift_row_single(chunk, 1);
            shift_row_single(chunk, 2);
            shift_row_single(chunk, 3);
        }
    }
    pub fn mix_columns(&mut self) {
        for element in self.array.iter_mut() {
            mix_columns(element);
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
            self.sub_bytes();
            self.shift_rows();
            self.mix_columns();
            self.add_round_key(&expanded_keys[i]);
        }
        self.sub_bytes();
        self.shift_rows();
        self.add_round_key(&expanded_keys.last().unwrap());

        for row in &self.array {
            for &element in row {
                print!("{:02x}", element);
            }
        }
    }
}

fn xor_matrix(first : &mut [[u8; 4]], second: &[[u8; 4]]) {
    for (index, element) in first.iter_mut().enumerate() {
        for (sub_index, sub_element) in element.iter_mut().enumerate() {
            *sub_element = *sub_element ^ second[index][sub_index];
        }
    }
}
fn shift_row_single(matrix: &mut [[u8; 4]], index: usize) {
    let one = matrix[0][index];
    let two = matrix[1][index];
    let three = matrix[2][index];
    let four = matrix[3][index];

    matrix[((0 + 4 - index) % 4) as usize][index] = one;
    matrix[((1 + 4 - index) % 4) as usize][index] = two;
    matrix[((2 + 4 - index) % 4) as usize][index] = three;
    matrix[((3 + 4 - index) % 4) as usize][index] = four;
}