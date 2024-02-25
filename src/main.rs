// Modules
mod process_command;

// Uses
use clap::{error::ErrorKind, CommandFactory, Parser};
use process_command::process_command;
use anyhow::Result;

// Defaults
const DEFAULT_PATH: &str = "random.txt";
const DEFAULT_WORDS_PER_LINE: u64 = 10;
const DEFAULT_AMOUNT_OF_LINES: u64 = 10;

// Struct for arguments
#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = String::from(DEFAULT_PATH))]
    path: String,
    #[arg(short, long, default_value_t = DEFAULT_WORDS_PER_LINE)]
    words_per_line: u64,
    #[arg(short, long, default_value_t = DEFAULT_AMOUNT_OF_LINES)]
    amount_of_lines: u64,
}

// Main function
fn main() -> Result<()> {
    // Parse the arguments
    let args: Args = Args::parse();

    // Process the command, if an error occurs, format it the same way as 'clap'
    if let Err(error) = process_command(args) {
        let mut cmd = Args::command();
        if let Some(source) = error.source() {
            cmd.error(ErrorKind::Io, format!("{}\n\ncause: {}", &error, source)).exit()
        } else {
            cmd.error(ErrorKind::Io, format!("{}", &error)).exit()
        }
    }

    // Return OK
    Ok(())
}
