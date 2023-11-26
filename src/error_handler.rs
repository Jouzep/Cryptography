use std::io::{self, Read};


use crate::rsa::{rsa::convert_little_endian};

pub fn get_message() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    let result = buffer
        .trim()
        .trim_matches(|c| c == '"' || c == '\r' || c == '\n')
        .to_string();
    result
}

fn xor_aes_error(algo: &str, message: &String, block: bool, mode: &str, content: &String) -> Result<(), &'static str> {
    let message_list = vec!["-c", "-d"];

    if !message_list.contains(&mode) {
        println!("Error");
        return Err("Not a good crypt mode");
    }

    if block && (message.len() != content.len()) {
        println!("error");
        return Err("Block mode error");
    }

    Ok(())
}

fn is_hexadecimal(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(16))
}

fn rsa_error<'a>(args: &'a [String], buffer:&String) -> Result<(), &'static str> {
    let message_list = vec!["-c", "-d", "-g"];

    if args.len() < 4 || args.len() > 5 {
        return Err("Wrong number of arg")
    }

    if !message_list.contains(&args[2].as_str()) {
        println!("Error");
        return Err("Wrong rsa mode")
    }


    if args[2] == "-g" {
        let p = convert_little_endian(args[3].clone());
        let q = convert_little_endian(args[4].clone());

        if !is_hexadecimal(&p) || !is_hexadecimal(&q) {
            return Err("Invalid hexadecimal representation for p or q");
        }
    } else {
        if args.len() != 4 {
            return Err("Wrong nb of args");
        }
        let key: Vec<&str> = args[2].split("-").collect();
        if buffer.len() < key[1].len() {
            return Err("Message can't be smaller than p or q")
        }
    }
    Ok(())
}

fn pgp_error<'a>(args: &'a [String]) -> Result<(), &'static str> {
    let message_list = vec!["-c", "-d"];


    if args.len() != 4 {
        return Err("Wrong nb of args");
    }
    if !message_list.contains(&args[2].as_str()) {
        return Err("Wrong pgp flag");
    }
    Ok(())
}

pub fn error_handler<'a>(args: &'a [String]) -> Result<(&'a [String], String), &'static str> {
    let buffer: String = if args[1] == "-rsa" && args[2] == "-g" {
        "".to_string()
    } else {
        get_message()
    };

    if args.len() >= 4 && args.len() <= 5 {
        return match args[1].as_str() {
            "-pgp" => pgp_error(args).map(|()| (args, buffer)),
            "-rsa" => rsa_error(args, &buffer).map(|()| (args, buffer)),
            "-aes" | "-xor" => {
                let block = args.contains(&"-b".to_string());
                xor_aes_error(&args[1].as_str(), &buffer, block, &args[2].as_str(), &args[4]).map(|()| (args, buffer))
            }
            _ => Err("Error in crypt flag"),
        };
    } else {
        println!("Not sufficient args");
        Err("Not sufficient args")
    }
}