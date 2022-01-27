// defines the basic ducky error message
extern crate colored;
use colored::*;
use crate::transpiler::ARGS;

pub struct DuckyError {
    message: String,
    data: String,
    verbose_info: String
}

// creates a new ducky error
impl DuckyError {
    
    pub fn new(message: &str, data: Option<&str>, verbose_info: (&str, bool)) -> Self {
        let mut err_data: &str = "";
        let mut err_info: &str = "";
        if data != None { err_data = data.unwrap() }
        if verbose_info.1 { err_info = verbose_info.0 }

        DuckyError { 
            message: message.to_string(),
            data: err_data.to_string(),
            verbose_info: err_info.to_string()
        }
    }
}

use std::fmt;
impl fmt::Display for DuckyError {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| {}", self.message.red().bold())
    }
}

// change this for prettier errors
impl fmt::Debug for DuckyError {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.verbose_info == "" { write!(f, "{} {}\n| {}", "[FATAL]".red().bold(), self.message, self.data) }
        else { write!(f, "{} {}\n| {}\n| {}", "[FATAL]".red().bold(), self.message, self.data, self.verbose_info.red().bold()) }
    }
}

// I know what this is supposed to do, but I never have to use it and can't remove it without breaking
impl std::error::Error for DuckyError {
    
    fn description(&self) -> &str {
        &self.message
    }
}

use std::io::{self};

impl From<io::Error> for DuckyError {
    
    fn from(error: io::Error) -> Self {
        
        let error_data = format!("{}", error);
        let file_message: String = format!("Attempted to open: '{}'", ARGS.payload_file.red().bold());
        match error.kind() {
            
            io::ErrorKind::NotFound => DuckyError::new(
                "Failed to locate given file.",
                Some(file_message.as_str()),
                (error_data.as_str(), ARGS.verbose)
            ),

            io::ErrorKind::PermissionDenied => DuckyError::new(
                "Failed to open file due to improper permissions.",
                Some(file_message.as_str()),
                (error_data.as_str(), ARGS.verbose)
            ),

            io::ErrorKind::UnexpectedEof => DuckyError::new(
                "Failed to parse payload due to an unexpected end of file.",
                Some(file_message.as_str()),
                (error_data.as_str(), ARGS.verbose)
            ),

            io::ErrorKind::InvalidData => DuckyError::new(
                "Failed to parse file due to invalid data.",
                Some(file_message.as_str()),
                (error_data.as_str(), ARGS.verbose)
            ),

            io::ErrorKind::Interrupted => DuckyError::new(
                "Failed to parse file due to being interrupted.",
                Some(file_message.as_str()),
                (error_data.as_str(), ARGS.verbose)
            ),

            io::ErrorKind::OutOfMemory => DuckyError::new(
                "Failed to parse file due to running out of availible memory",
                Some(file_message.as_str()),
                (error_data.as_str(), ARGS.verbose)
            ),

            _ => DuckyError::new(
                "Failed to run program due to unknown error.",
                Some("Make sure to not pass directories as arguments."),
                (error_data.as_str(), ARGS.verbose)
            )
        }
    }
}

impl From<serde_json::Error> for DuckyError {
    
    fn from(error: serde_json::Error) -> Self {
        let error_data = format!("{}", error);
        let layout_file: String = format!("Provided Layout file: '{}'", ARGS.keyboard_language);
        
        DuckyError::new(
            "Failed to parse the provided Keyboard Layout file",
            Some(layout_file.as_str()),
            (error_data.as_str(), ARGS.verbose)
        )
    }
}

impl From<core::num::ParseIntError> for DuckyError {
    
    fn from(error: core::num::ParseIntError) -> Self {
        let error_data = format!("{}", error);
        let layout_file: String = format!("Provided Layout file: '{}'", ARGS.keyboard_language);
        
        DuckyError::new(
            "Unable to convert Keyboard Layout element into valid keycode.",
            Some(layout_file.as_str()),
            (error_data.as_str(), ARGS.verbose)
        )
    }
}