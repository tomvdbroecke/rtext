// Uses
use random_word::Lang;
use crate::Args;
use std::{fs::File, io::{BufWriter, Write}, time::Instant};
use num_format::{Locale, ToFormattedString};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle, ProgressState, DecimalBytes};
use human_duration::human_duration;

// Process command function
pub(crate) fn process_command(args: Args) -> Result<(), anyhow::Error> {
    // Start keeping track of time
    let start_time: Instant = Instant::now();

    // Start progress bar (amount of increments equal to amount of lines)
    let pb: ProgressBar = ProgressBar::new(args.lines);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} lines ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{}", human_duration(&state.eta())).unwrap())
        .progress_chars("#>-"));

    // Create file
    let file: File = File::create(&args.path)?;

    // Wrap file in a BufWriter
    let mut buf_writer: BufWriter<&File> = BufWriter::new(&file);

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

        // Increment progress bar by 1
        pb.inc(1);

        // Go to next line
        l = l + 1;
    }

    // Make sure progress bar is complete
    pb.finish();

    // User feedback
    let s: &str = "Success!";
    println!(
        "{} {} {} was generated in {} with {} words split into {} lines of {} words each",
        s.bold().green(),
        &args.path.bold(),
        format!("({})", DecimalBytes(file.metadata().unwrap().len())).bold(),
        format!("{}", human_duration(&start_time.elapsed())).bold(),
        (&args.lines * &args.words_per_line).to_formatted_string(&Locale::en).bold(),
        &args.lines.to_formatted_string(&Locale::en).bold(),
        &args.words_per_line.to_formatted_string(&Locale::en).bold()
    );

    // Return OK
    Ok(())
}