// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
use serde_json::{Value};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::DuckyError;

use super::{Args, KeyValue, KeyReport, RELEASE};

#[derive(Debug)]
struct KeyCode {
    key_code: KeyValue,
    mod_code: KeyValue
}

trait ToKeyCode {
    fn to_keycode(&self, kb_layout: &Value) -> Result<KeyCode, DuckyError>;
    fn is_mod(&self) -> bool;
}

// responsible for converting payload tokens to keycodes
impl <'a>ToKeyCode for &'a str {

    fn is_mod(&self) -> bool {
        match *self {
            "CONTROL" => true,
            "CTRL" => true,
            "SHIFT" => true,
            "ALT" => true,
            "SUPER" => true,
            "WINDOWS" => true,
            "WIN" => true,
            "GUI" => true,
            _ => false
        }
    }

    fn to_keycode(&self, kb_layout: &Value) -> Result<KeyCode, DuckyError> {
        let mut keycode = kb_layout[&self].to_string().parse::<KeyValue>()?;
        let mut modcode = 0;
        let cmd_type = &self.is_mod();

        if keycode > 128 { keycode = keycode - 128; modcode = kb_layout["SHIFT"].to_string().parse::<KeyValue>()?; }
        if cmd_type == &true { modcode = keycode; keycode = 0; }

        Ok(KeyCode {
            key_code: keycode,
            mod_code: modcode
        })
    }

}

// loads the [kb_language].json file from the provided directory
fn load_layout(kb_lang: &str, v: &bool) -> Result<Value, DuckyError> {
    let file_location: String = format!("{}", &kb_lang);
    // added the match statement for more verbose errors
    let layout_file = match File::open(&file_location) {
        Ok(f) => Ok(f),
        Err(e) => {
            let verbose_error = format!("{}", e);
            Err(DuckyError::new("Failed to open provided Keyboard Layout file.", Some(format!("Attempted to open: {}", &file_location).as_str()), (verbose_error.as_str(), *v)))
        }
    }?;
    let mut bufreader = BufReader::new(layout_file);
    let mut layout_contents: String = String::new();
    bufreader.read_to_string(&mut layout_contents)?;

    Ok(serde_json::from_str(&layout_contents)?)
}

pub struct Parser {
    kb_layout: Value,
    args: Args
}

impl Parser {
    pub fn new(args: &Args) -> Result<Self, DuckyError> {
        
        let new_args: Args = args.clone();

        Ok(Parser {
            kb_layout: load_layout(&args.keyboard_language, &args.verbose)?,
            args: new_args
        })
    }

    pub fn parse_payload(&self) -> Result<Vec<KeyReport>, DuckyError> {
        let payload_file: String = format!("{}", &self.args.payload_file);
        let file = File::open(&payload_file)?;
        let reader = BufReader::new(file);

        let mut report_buf: Vec<KeyReport> = Vec::new();
        let mut d_delay: bool = false;
        
        for (index, line) in reader.lines().enumerate() {
            let payload_line = line?;
            let payload_tokens = payload_line.split_whitespace().collect::<Vec<&str>>();
            let err_data = format!("Error occurs in: '{}'\n| {}: '{}'", payload_file, index, payload_line);

            if payload_tokens.len() == 0 { continue }

            match payload_tokens[0] {
                "REM" => continue,
                
                "STRING" => {
                    let report = match Self::string_command(&self, payload_tokens[1..].to_vec()) {
                        Ok(r) => Ok(r),
                        Err(_e) => Err(DuckyError::new(
                            "Unable to convert Payload element into valid keycode.",
                            Some(err_data.as_str()),
                            ("Make sure this command is present in your [keyboard_language].json file.", self.args.verbose)
                        ))
                    }?; 
                    for r in report { report_buf.push(r); }
                },
                
                "DELAY" => {                
                    // an empty delay means we default to default_delay
                    if d_delay == true && payload_tokens.len() <= 1 { report_buf.push([100, 0, 0, 0, 0, 0, 0, 0])}
                    else if d_delay == false && payload_tokens.len() <= 1 {
                        Err(DuckyError::new(
                            "DELAY was called without a value, but DEFAULT_DELAY was not set.",
                            Some(err_data.as_str()),
                            ("Make sure to call DEFAULT_DELAY before assigning empty delays.", self.args.verbose)
                        ))?
                    }
                    
                    else {
                        let delay_time = match payload_tokens[1].parse::<KeyValue>() {
                        Ok(t) => Ok(t),
                        Err(_e) => Err(DuckyError::new(
                            "Failed to convert delay time into a number.",
                            Some(err_data.as_str()),
                            ("Make sure the value for delay time is formatted as such: DELAY [delay_time]", self.args.verbose)
                            ))
                        }?;
                        report_buf.push([100, 0, delay_time, 0, 0, 0, 0, 0])
                    }
                },
                
                "DEFAULT_DELAY" => {
                    let delay_time = match payload_tokens[1].parse::<KeyValue>() {
                        Ok(t) => Ok(t),
                        Err(_e) => Err(DuckyError::new(
                            "Failed to convert default delay time into a number.",
                            Some(err_data.as_str()),
                            ("Make sure the value for delay time is formatted as such: DEFAULT_DELAY [delay_time]", self.args.verbose)
                        ))
                    }?;
                    d_delay = true;
                    report_buf.push([200, 0, delay_time, 0, 0, 0, 0, 0])
                },
                
                _ => { 
                    let report = match Self::mod_command(&self, payload_tokens) {
                        Ok(r) => Ok(r),
                        Err(_e) => Err(DuckyError::new(
                            "Unable to convert Payload element into valid keycode.",
                            Some(err_data.as_str()),
                            ("Make sure this command is present in your [keyboard_language].json file.", self.args.verbose)
                        ))
                    }?; 
                    for r in report { report_buf.push(r); }
                }
            }
        }

        Ok(report_buf)
    }

    fn mod_command(&self, payload_tokens: Vec<&str>) -> Result<Vec<KeyReport>, DuckyError> {
        // converts tokens into the keycode struct and collects them in a vector
        let mut keycode_buf: Vec<KeyCode> = Vec::new();
        for token in payload_tokens {
            keycode_buf.push(token.to_string()
                .as_str()
                .to_keycode(&self.kb_layout)?
            );
        }

        let mut report_buf: Vec<KeyReport> = Vec::new();
        // each report is only 128 bytes in length due to delay reports
        let mut report: KeyReport = RELEASE;

        // creates a report for each keycode
        // sorting algorithm will go here once I am big brain enough
        for keycode in keycode_buf {
            report[0] = keycode.mod_code;
            report[2] = keycode.key_code;
            report_buf.push(report);
            report = RELEASE;
        }
        report_buf.push(RELEASE);

        Ok(report_buf)
    }

    fn string_command(&self, payload_tokens: Vec<&str>) -> Result<Vec<KeyReport>, DuckyError> {
        
        let mut keycode_buf: Vec<KeyCode> = Vec::new();
        for token in payload_tokens.join(" ").chars() {
            keycode_buf.push(token.to_string()
                .as_str()
                .to_keycode(&self.kb_layout)?
            );
        }

        let mut report_buf: Vec<KeyReport> = Vec::new();
        // each report is only 128 bytes in length due to delay reports
        let mut report: KeyReport = RELEASE;
        let mut x: bool = true;
        // this sorts keycodes into a buffer of reports based on a few rules
        // 1. The mod key is in the 0th element and affects the entire report
        // 2. You can send up to 5 key codes at once in the 2nd-8th element of the report
        // 3. There cannot be duplicate keycodes in the report (USB protocol will see it as an error)
        // 4. The keys will remain pressed until a release report is sent
        
        // if there is only one element in the keycode buffer
        if keycode_buf.len() == 1 { report[0] = keycode_buf[0].mod_code; report[2] = keycode_buf[0].key_code; report_buf.push(report); report_buf.push(RELEASE); return Ok(report_buf); }
        
        for (i, keycode) in keycode_buf.iter().enumerate() {
            // if this is the first report in the buffer
            if x { report[0] = keycode.mod_code; report[2] = keycode.key_code; x = false; continue; }
            // if the report is full
            if report[report.len() - 1] != 0 { report_buf.push(report); report = RELEASE; report_buf.push(report); }
            // if the mod code for the key is != to the report mod code
            else if report[0] != keycode.mod_code { report_buf.push(report); report = RELEASE; report_buf.push(report); }
            // if the key code is already present in the report
            else if report.contains(&keycode.key_code) { report_buf.push(report); report = RELEASE; report_buf.push(report); }

            // set the report mod code to the given mod code
            report[0] = keycode.mod_code;
            // push keycode to report
            // could be optimized but this transpiles in seconds
            for r in report[2..].iter_mut() {
                // if the report index is already set
                if r != &0_u16 { continue }
                // we push the keycode to the report
                *r = keycode.key_code;
                break;
            }

            // if this is the last report
            // also this -1 here was the result of 30 mins of debugging...
            if i == keycode_buf.len() - 1 { report_buf.push(report); report_buf.push(RELEASE); }

        }
        Ok(report_buf)
    }

}