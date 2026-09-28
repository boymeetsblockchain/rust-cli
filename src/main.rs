use std::env;

use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]

struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments passed. Expected 4 m got {}",
            "Error".red().bold(),
            args.len()
        )
    }
}

fn print_help() {
    println!("");
    eprintln!(
        "{}- replace a string with a new string",
        "find and replace".green()
    );
    eprint!("Usage:<target string> <replacement string> <INPUT FILE> <OUTPUT FILE>")
}
