pub fn error_handler(args: Vec<String>) -> Result<Vec<String>, &'static str> {
    let algo_list = vec!["-xor", "-aes", "-rsa", "-pgp"];

    if !algo_list.contains(&args[1].as_str()) {
        return Err("Wrong algo arg")
    }
    Ok(args)
}