use std::io::{self, Read};

pub fn get_message() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
        .expect("Failed to read from stdin");
    let buff = &buffer;
    let buff = buff.trim_end();
    let buff = buff.trim_end_matches("\\r\\n");
    let result = buff.to_string();
    result
}

fn xor_aes_error(algo: &str, message: &str, block: bool, key: &str, content: &String) -> bool {
    let message_list = vec!["-c", "-d"];

    if algo == "-xor" && (!block || !message_list.contains(&message) || algo == "-xor" && content.len() != key.len()) {
        return true;
    }
    false
}

pub fn error_handler<'a>(args: &'a [String]) -> Result<(&'a [String], String), &'static str> {
    let buffer: String = if args[1] == "-rsa" && args[2] == "-g" { "".to_string() } else {  get_message()};

    if let [_, algo, message, block, key] = args {
        if xor_aes_error(algo, &message, block == "-b", key, &buffer) {
            return Err("Error in XOR or AES validation");
        }
    } else {
        println!("Not sufficient args");
    }
    Ok((args, buffer))
}
