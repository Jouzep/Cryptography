use std::io::{self, Read};

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

    if (algo == "-xor" || block) && message.len() != content.len() {
        println!("error");
        return Err("Block mode error");
    }

    Ok(())
}

fn rsa_error<'a>(args: &'a [String]) -> Result<(), &'static str> {
    let message_list = vec!["-c", "-d", "-g"];

    if !message_list.contains(&args[2].as_str()) {
        println!("Error");
        return Err("Wrong rsa mode")
    }

    Ok(())
}

pub fn error_handler<'a>(args: &'a [String]) -> Result<(&'a [String], String), &'static str> {
    let buffer: String = if args[1] == "-rsa" && args[2] == "-g" {
        "".to_string()
    } else {
        get_message()
    };

    if args.len() > 4 {
        return match args[1].as_str() {
            "-rsa" => rsa_error(args).map(|()| (args, buffer)),
            "-aes" | "-xor" => {
                let block = args[3].parse().unwrap_or(false);
                xor_aes_error(&args[1].as_str(), &buffer, block, &args[2].as_str(), &args[4]).map(|()| (args, buffer))
            }
            _ => Err("Error in crypt flag"),
        };
    } else {
        println!("Not sufficient args");
        // Decide what to return in this case
        // Returning an Err for now
        Err("Not sufficient args")
    }
}