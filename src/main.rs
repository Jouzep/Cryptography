use std::env;
use  std::process::ExitCode;
mod error_handler;
mod my_pgp;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match error_handler::error_handler(args) {
        Ok(arg) => {
            my_pgp::run_pgp(arg);
        }
        Err(err) => {
            println!("Error: {}", err);
            return ExitCode::from(84);
        }
    };
    return ExitCode::SUCCESS
}
