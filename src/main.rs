// Uses
use clap::Parser;

// Defaults
const DEFAULT_PATH: &str = "random.txt";
const DEFAULT_WORDS_PER_LINE: u64 = 10;
const DEFAULT_AMOUNT_OF_LINES: u64 = 10;

// Struct for arguments
#[derive(Parser)]
struct Args {
    #[arg(default_value_t = String::from(DEFAULT_PATH))]
    path: String,
    #[arg(default_value_t = DEFAULT_WORDS_PER_LINE)]
    words_per_line: u64,
    #[arg(default_value_t = DEFAULT_AMOUNT_OF_LINES)]
    amount_of_lines: u64,
}

// Main function
fn main() {
    println!("Hello, world!")
}
