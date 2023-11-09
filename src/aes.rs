extern crate hex;

use hex::FromHex;

fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let byte_array_result = Vec::from_hex(hex_string);
    byte_array_result
}