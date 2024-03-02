// Uses
use random_word::Lang;
use crate::Args;
use std::{fs::File, io::{BufWriter, Write}, path::Path, process, time::{Duration, Instant}};
use num_format::{Locale, ToFormattedString};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle, ProgressState, DecimalBytes};
use dialoguer::Confirm;
use anyhow::Error;

// Process command function
pub(crate) fn process_command(args: Args) -> Result<(), Error> {
    // Check if the file exists
    let _pass: Result<(), Error> = check_file_exists(&args);

    // Start keeping track of time
    let start_time: Instant = Instant::now();

    // Start progress bar (amount of increments equal to amount of lines)
    let pb: ProgressBar = ProgressBar::new(args.lines);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} lines ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{}", format_time(&state.eta())).unwrap())
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
        format!("{}", format_time(&start_time.elapsed())).bold(),
        (&args.lines * &args.words_per_line).to_formatted_string(&Locale::en).bold(),
        &args.lines.to_formatted_string(&Locale::en).bold(),
        &args.words_per_line.to_formatted_string(&Locale::en).bold()
    );

    // Return OK
    Ok(())
}

// Check file existence function
fn check_file_exists(args: &Args) -> Result<(), Error> {
    let exists: bool = Path::new(&args.path).exists();

    if exists {
        let confirmation = Confirm::new()
            .with_prompt(format!("The file {} already exists, do you want to overwrite this file?", &args.path.bold()))
            .default(true)
            .interact()
            .unwrap();
        
        if confirmation {
            Ok(())
        } else {
            process::exit(0)
        }
    } else {
        Ok(())
    }
}

// Format time function
fn format_time(&duration: &Duration) -> String {
    let seconds = duration.as_secs();
    let days = seconds / (24 * 3600);
    let hours = (seconds % (24 * 3600)) / 3600;
    let minutes = (seconds % 3600) / 60;
    let remaining_seconds = seconds % 60;

    match (days, hours, minutes, remaining_seconds) {
        (0, 0, 0, s) => format!("{}s", s),
        (0, 0, m, s) => format!("{}m {}s", m, s),
        (0, h, m, s) => format!("{}h {}m {}s", h, m, s),
        (d, h, m, s) => format!("{}d {}h {}m {}s", d, h, m, s),
    }
}