use crate::aes::aes_key_struct::sub_bytes;

#[derive(Debug)]
pub struct AesMessage {
    pub array: Vec<[u8; 4]>,
}

use std::fmt;

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
        let modulos = 4 - double_array.len() % 4;
        for i in 0..modulos {
            double_array.push([0; 4]);
        }
        let mut data : Vec<[u8; 4]> = vec![];
        data.push(    [0x19, 0x3d, 0xe3, 0xbe]);
        data.push(    [0xa0, 0xf4, 0xe2, 0x2b]);
        data.push(    [0x9a, 0xc6, 0x8d, 0x2a]);
        data.push(    [0xe9, 0xf8, 0x48, 0x08]);
        AesMessage {
            array: data
        }
    }

    pub fn sub_bytes(&mut self) {
        for element in self.array.iter_mut() {
            for sub_element in element.iter_mut() {
                if *sub_element != 0 {
                    *sub_element = sub_bytes(&sub_element);
                }
            }
        }
    }

    pub fn shift_rows(&mut self) {
        for (index, chunk) in self.array.chunks_mut(4).enumerate() {
            shift_row_single(chunk, 1);
            shift_row_single(chunk, 2);
            shift_row_single(chunk, 3);
            println!("Matrix {} 4x4 : {:?}", index, chunk);
        }
    }
}

fn shift_row_single(matrix: &mut [[u8; 4]], index: usize) {
    let one = matrix[0][index];
    let two = matrix[1][index];
    let three = matrix[2][index];
    let four = matrix[3][index];

    // println!("{}", (0 - index as i32).abs() as usize);
    // (0 + 4 - 3) % 4;
    matrix[((0 + 4 - index) % 4) as usize][index] = one;
    matrix[((1 + 4 - index) % 4) as usize][index] = two;
    matrix[((2 + 4 - index) % 4) as usize][index] = three;
    matrix[((3 + 4 - index) % 4) as usize][index] = four;
}