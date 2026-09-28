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
    eprintln!(
        "{}- replace a string with a new string",
        "find and replace".green()
    );
    eprint!("Usage:<target string> <replacement string> <INPUT FILE> <OUTPUT FILE>")
}
