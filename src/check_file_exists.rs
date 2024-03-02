// Uses
use std::{path::Path, process};
use crate::Args;
use anyhow::Error;
use colored::Colorize;
use dialoguer::Confirm;

// Check file existence function
pub(crate) fn check_file_exists(args: &Args) -> Result<(), Error> {
    // Check if the file exists
    let exists: bool = Path::new(&args.path).exists();

    // If the file exists, ask for overwrite confirmation, otherwise exit the program
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