use crate::aes::aes_constant::{INV_S_BOX, S_BOX};

pub fn xor_matrix(first : &mut [[u8; 4]], second: &[[u8; 4]]) {
    for (index, element) in first.iter_mut().enumerate() {
        for (sub_index, sub_element) in element.iter_mut().enumerate() {
            *sub_element = *sub_element ^ second[index][sub_index];
        }
    }
}

pub fn shift_row_single(matrix: &mut [[u8; 4]], index: usize) {
    let one = matrix[0][index];
    let two = matrix[1][index];
    let three = matrix[2][index];
    let four = matrix[3][index];

    matrix[((0 + 4 - index) % 4) as usize][index] = one;
    matrix[((1 + 4 - index) % 4) as usize][index] = two;
    matrix[((2 + 4 - index) % 4) as usize][index] = three;
    matrix[((3 + 4 - index) % 4) as usize][index] = four;
}

pub fn inv_shift_row_single(matrix: &mut [[u8; 4]], index: usize) {
    let one = matrix[0][index];
    let two = matrix[1][index];
    let three = matrix[2][index];
    let four = matrix[3][index];

    matrix[((0 + index) % 4) as usize][index] = one;
    matrix[((1 + index) % 4) as usize][index] = two;
    matrix[((2 + index) % 4) as usize][index] = three;
    matrix[((3 + index) % 4) as usize][index] = four;
}

pub fn euclidean_division(dividend: &u8, divisor: u8) -> (u8, u8) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

pub fn sub_bytes(number: &u8) -> u8 {
    let (first, second) = euclidean_division(number, 16);
    return S_BOX[first as usize][second as usize]
}

pub fn inv_sub_bytes(number: &u8) -> u8 {
    let (first, second) = euclidean_division(number, 16);
    return INV_S_BOX[first as usize][second as usize]
}