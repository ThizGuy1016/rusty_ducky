extern crate colored;

use colored::*;
use std::fs;
use std::io::Write;

use crate::DuckyError;
use super::ARGS;

pub fn ducky_read_file(filename: &String) -> Result<String, DuckyError> {
    match fs::read_to_string(filename) {
        Ok(f) => Ok(f),
        Err(e) => {
            let err_data = format!("{}", e);
            let err_msg = format!("Attempted to open: '{}'", filename.red().bold());

            Err(DuckyError::new(
                "Unable to locate given file.",
                Some(err_msg.as_str()),
                (err_data.as_str(), ARGS.verbose)
            ))
        } 
    }
}

pub fn ducky_write_file(filename: &String, content: &String) -> Result<(), DuckyError> {
    let mut file = match fs::File::create(filename) {
        Ok(f) => Ok(f),
        Err(e) => {
            let err_data = format!("{}", e);
            let err_msg = format!("Attempted to create: '{}'", filename.red().bold());
        
            Err(DuckyError::new(
                "Unable to create given file.",
                Some(err_msg.as_str()),
                (err_data.as_str(), ARGS.verbose)
            ))
        }
    }?;

    Ok(match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_data = format!("{}", e);
            let err_msg = format!("Attempted to write to: '{}'", filename.red().bold());
        
            Err(DuckyError::new(
                "Unable to write to given file.",
                Some(err_msg.as_str()),
                (err_data.as_str(), ARGS.verbose)
            ))
        }
    }?)
}