use std::{env, process::exit};
pub mod a25;
pub mod error;
pub mod lex;
pub mod location;
pub mod parse;
pub mod token;

fn usage() -> ! {
    eprintln!(
        "Usage: {} <filename>",
        env::current_exe()
            .expect("Failed to get executable path")
            .display()
    );
    exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage()
    }
    if let Err(e) = a25::run_file(&args[1]) {
        eprintln!("{e}");
        exit(1);
    }
}
