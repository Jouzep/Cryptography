use std::env;
use std::process::ExitCode;
mod error_handler;
mod my_pgp;
mod print_usage;
mod aes {
    pub mod aes_module;
    pub mod aes_key_struct;
    pub mod aes_constant;
    pub mod aes_message;
    pub mod aes_function;
}
mod rsa {
    pub mod rsa;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "-h" {
        print_usage::print_usage();
        return ExitCode::SUCCESS;
    }
    let args_ref = args;
    match error_handler::error_handler(&args_ref) {
        Ok((arg, buffer)) => {
            my_pgp::run_pgp( args_ref, buffer);
        }
        Err(err) => {
            println!("Error: {}", err);
            return ExitCode::from(84);
        }
    };
    return ExitCode::SUCCESS
}
