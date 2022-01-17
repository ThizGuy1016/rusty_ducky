// defines the basic ducky error message
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

        DuckyError { message: message.to_string(), data: err_data.to_string(), verbose_info: err_info.to_string() }
    }
}

use std::fmt;
impl fmt::Display for DuckyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| {}", self.message)
    }
}

// change this for prettier errors
impl fmt::Debug for DuckyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n| {}\n| {}", self.message, self.data, self.verbose_info)
    }
}

impl std::error::Error for DuckyError {
    fn description(&self) -> &str {
        &self.message
    }
}

use std::io::{self};
impl From<io::Error> for DuckyError {
    fn from(error: io::Error) -> Self {
        let error_data = format!("{}", error);
        match error.kind() {
            io::ErrorKind::NotFound => DuckyError::new("Failed to locate payload file.", None, (error_data.as_str(), true)),
            io::ErrorKind::PermissionDenied => DuckyError::new("Failed to open payload file due to improper permissions.", None, (error_data.as_str(), true)),
            io::ErrorKind::UnexpectedEof => DuckyError::new("Failed to parse payload file due to an unexpected end of file.", None, (error_data.as_str(), true)),
            io::ErrorKind::InvalidData => DuckyError::new("Failed to parse payload file due to invalid data.", None, (error_data.as_str(), true)),
            io::ErrorKind::Interrupted => DuckyError::new("Failed to parse payload file due to being interrupted.", None, (error_data.as_str(), true)),
            io::ErrorKind::OutOfMemory => DuckyError::new("Failed to parse payload file due to running out of availible memory", None, (error_data.as_str(), true)),
            _ => DuckyError::new("Failed to run program due to unknown error.", Some("Make sure to not pass directories as arguments."), (error_data.as_str(), true))
        }
    }
}

impl From<serde_json::Error> for DuckyError {
    fn from(error: serde_json::Error) -> Self {
        let error_data = format!("{}", error);
        DuckyError::new("Failed to parse the provided Keyboard Layout file", None, (error_data.as_str(), true))
    }
}

impl From<core::num::ParseIntError> for DuckyError {
    fn from(error: core::num::ParseIntError) -> Self {
        let error_data = format!("{}", error);
        DuckyError::new("Unable to convert Keyboard Layout element into valid keycode.", None, (error_data.as_str(), true))
    }
}