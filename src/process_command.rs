// Uses
use random_word::Lang;
use crate::Args;
use std::{fs::File, io::{BufWriter, Write}};


// Process command function
pub(crate) fn process_command(args: Args) -> Result<(), anyhow::Error> {
    // Create file
    let file = File::create(&args.path)?;

    // Wrap file in a BufWriter
    let mut buf_writer = BufWriter::new(file);

    // Define variables to keep track of loop
    let mut l: u64 = 0;
    let mut w: u64;

    // Loop over amount of lines
    while l < args.lines {
        // Set word count (back) to 0
        w = 0;

        // Define line string
        let mut line: String = Default::default();

        // Loop over amount of words
        while w < args.words_per_line {
            // Generate random word and append it to the line
            let word: &str = random_word::gen(Lang::En);
            line.push_str(word);

            // On each but the final word, add a space
            if w + 1 < args.words_per_line {
                line.push_str(" ");
            }

            // Go to next word
            w = w + 1;
        }

        // Write line to file
        line.push_str("\n");
        buf_writer.write(line.as_bytes())?;

        // Go to next line
        l = l + 1;
    }

    // Return OK
    Ok(())
}