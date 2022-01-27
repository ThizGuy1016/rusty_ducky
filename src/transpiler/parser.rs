// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, prelude::*};

extern crate colored;
use colored::*;

use crate::DuckyError;

use super::{ARGS, KeyValue, KeyReport, RELEASE, ducky_read_file};

#[derive(Debug)]
struct KeyCode {
    key_code: KeyValue,
    mod_code: KeyValue
}

fn is_mod<T: ToString>(s: &T) -> bool {
    match s.to_string().as_str() {
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

trait ToKeyCode {
    fn to_keycode<T>(&self, kb_layout: &Value) -> Result<KeyCode, DuckyError>;
}

impl <T>ToKeyCode for T
where T: serde_json::value::Index + Sized + Clone + std::fmt::Display
{
    fn to_keycode<S>(&self, kb_layout: &Value) -> Result<KeyCode, DuckyError> {

        let mut keycode = kb_layout[&self].to_string().parse::<KeyValue>()?;
        let mut modcode = 0;
        let cmd_type = is_mod(self);

        if keycode > 128 { keycode = keycode - 128; modcode = kb_layout["SHIFT"].to_string().parse::<KeyValue>()?; }
        if cmd_type { modcode = keycode; keycode = 0; }

        Ok(KeyCode {
            key_code: keycode,
            mod_code: modcode
        })
    }
}

pub struct Parser {
    kb_layout: Value
}

impl Parser {

    // loads the [kb_language].json file from the provided directory
    pub fn new() -> Result<Parser, DuckyError> {
        let layout_contents = ducky_read_file(&ARGS.keyboard_language)?;
        Ok( Self { kb_layout: serde_json::from_str(&layout_contents)? } )
    }

    pub fn parse_payload(&self) -> Result<Vec<KeyReport>, DuckyError> {
        let payload_file: String = format!("{}", &ARGS.payload_file);
        let file = File::open(&payload_file)?;
        let reader = BufReader::new(file);

        let mut report_buf: Vec<KeyReport> = Vec::new();
        let mut d_delay: bool = false;
        
        for (index, line) in reader.lines().enumerate() {
            let payload_line = line?;
            let payload_tokens = payload_line.split_whitespace().collect::<Vec<&str>>();
            let err_data = format!("Error occurs in: '{}'\n| {}: '{}'", payload_file.yellow().bold(), (index + 1).to_string().yellow(), payload_line.red().bold());

            if payload_tokens.len() == 0 { continue }

            match payload_tokens[0] {
                "REM" => continue,
                
                "STRING" => {
                    let report = match Self::string_command(&self, payload_tokens[1..].to_vec()) {
                        Ok(r) => Ok(r),
                        Err(_e) => Err(DuckyError::new(
                            "Unable to convert Payload element into valid keycode.",
                            Some(err_data.as_str()),
                            ("Make sure this command is present in your [keyboard_language].json file.", ARGS.verbose)
                        ))
                    }?; 
                    for r in report { report_buf.push(r); }
                },
                
                "DELAY" => {                
                    // an empty delay means we default to default_delay
                    if d_delay == true && payload_tokens.len() <= 1 { report_buf.push([0, 100, 0, 0, 0, 0, 0, 0])}
                    else if d_delay == false && payload_tokens.len() <= 1 {
                        Err(DuckyError::new(
                            format!("{} was called without a value, but {} was not set.", "DELAY".yellow().bold(), "DEFAULT_DELAY".blue().bold()).as_str(),
                            Some(err_data.as_str()),
                            ("Make sure to call DEFAULT_DELAY before assigning empty delays.", ARGS.verbose)
                        ))?
                    }
                    
                    else {
                        let delay_time = match payload_tokens[1].parse::<KeyValue>() {
                        Ok(t) => Ok(t),
                        Err(_e) => Err(DuckyError::new(
                            format!("Failed to convert {} time into a number.", "DELAY".yellow().bold()).as_str(),
                            Some(err_data.as_str()),
                            ("Make sure the value for delay time is formatted as such: DELAY [delay_time]", ARGS.verbose)
                            ))
                        }?;
                        report_buf.push([0, 100, delay_time, 0, 0, 0, 0, 0])
                    }
                },
                
                "DEFAULT_DELAY" => {
                    if payload_tokens.len() == 1 { 
                        return Err(DuckyError::new(
                            format!("No value given for {}", "DEFAULT_DELAY".yellow().bold()).as_str(),
                            Some(err_data.as_str()),
                            ("Make sure the value for delay time is formatted as such: DEFAULT_DELAY [delay_time]", ARGS.verbose)
                        ))
                    }
                    let delay_time = match payload_tokens[1].parse::<KeyValue>() {
                        Ok(t) => Ok(t),
                        Err(_e) => Err(DuckyError::new(
                            format!("Failed to convert {} time into a number.", "DEFAULT_DELAY".yellow().bold()).as_str(),
                            Some(err_data.as_str()),
                            ("Make sure the value for delay time is formatted as such: DEFAULT_DELAY [delay_time]", ARGS.verbose)
                        ))
                    }?;
                    d_delay = true;
                    report_buf.push([0, 200, delay_time, 0, 0, 0, 0, 0])
                },
                
                _ => { 
                    let report = match Self::mod_command(&self, payload_tokens) {
                        Ok(r) => Ok(r),
                        Err(_e) => Err(DuckyError::new(
                            "Unable to convert Payload element into valid keycode.",
                            Some(err_data.as_str()),
                            ("Make sure this command is present in your [keyboard_language].json file.", ARGS.verbose)
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
            keycode_buf.push(token
                .to_keycode::<&str>(&self.kb_layout)?
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
            keycode_buf.push(token
                .to_string()
                .to_keycode::<String>(&self.kb_layout)?
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