// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
use serde_json::{Value};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::DuckyError;

use super::{KeyCode, KeyReport, RELEASE, Args};

fn load_layout(args: &Args) -> Result<Value, DuckyError> {
    let layout_file = match File::open(&args.keyboard_language) {
        Ok(f) => Ok(f),
        Err(e) => {
            let verbose_error = format!("{}", e);
            Err(DuckyError::new("Failed to open provided Keyboard Layout file.", Some(&args.keyboard_language), (verbose_error.as_str(), args.verbose)))
        }
    }?;
    let mut bufreader = BufReader::new(layout_file);
    let mut layout_contents: String = String::new();
    bufreader.read_to_string(&mut layout_contents)?;

    Ok(serde_json::from_str(&layout_contents)?)
}

trait ToReport {
    fn to_report(&self, kb_layout: &Value) -> Result<KeyReport, DuckyError>;
    fn is_mod(&self) -> bool;
}

impl <'a>ToReport for &'a str {

    fn is_mod(&self) -> bool {
        match *self {
            "CONTROL" => true,
            "CTRL" => true,
            "SHIFT" => true,
            "ALT" => true,
            "WINDOWS" => true,
            "WIN" => true,
            "GUI" => true,
            _ => false
        }
    }

    fn to_report(&self, kb_layout: &Value) -> Result<KeyReport, DuckyError> {
        let mut report: KeyReport = RELEASE;
        let keycode = kb_layout[&self].to_string().parse::<KeyCode>()?;
        let cmd_type = &self.is_mod();

        if keycode > 128 { report[0] = kb_layout["SHIFT"].to_string().parse::<KeyCode>()?; report[2] = keycode - 128; }
        else if *cmd_type { report[0] = keycode }
        else { report[2] = keycode }

        Ok(report)
    }    
}

pub fn parse_payload(args: &Args) -> Result<Vec<KeyReport>, DuckyError> {
    let payload_file: String = format!("{}", &args.payload_file);
    let file = File::open(payload_file)?;
    let reader = BufReader::new(file);
    let kb_layout = load_layout(&args)?;

    // going to implement an error tracer that will display the line and index of any duckyscript errors
    let mut report_buf: Vec<KeyReport> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let payload_line = line?.to_string();
        let payload_tokens: Vec<&str> = payload_line.split(' ').collect();
        let err_data = format!("Error in: {}\n{}: {:?}", &args.payload_file, index + 1, payload_line);

        if &kb_layout[payload_tokens[0]] == &Value::Null { Err(DuckyError::new("Your payload contains an unkown command.", Some(err_data.as_str()), (payload_tokens[0], args.verbose)))? }

        let tmp_report = match payload_tokens[0] {
            "STRING" => {
                let mut tmp_buf: Vec<KeyReport> = Vec::new();
                let tokens: String = payload_tokens[1..].join(" "); 
                let token_buf: Vec<char> = tokens.chars().collect();
                for token in token_buf {
                    let token = match token.to_string().as_str().to_report(&kb_layout) {
                        Ok(t) => Ok(t),
                        Err(e) => {
                            let verbose_error = format!("{}", e);
                            Err(DuckyError::new("Unkown character found in given payload file!", Some(err_data.as_str()), (verbose_error.as_str(), args.verbose)))
                        }
                    }?;
                    tmp_buf.push(token);
                    tmp_buf.push(RELEASE);
                }
                tmp_buf
            },
            "REM" => { continue },
            "DELAY" => {
                if payload_tokens.len() == 1 { vec![[100, 0, 0, 0, 0, 0, 0, 0]] }
                else {
                    let delay_value: KeyCode = match payload_tokens[1].to_string().parse() {
                        Ok(t) => Ok(t),
                        Err(e) => {
                            let verbose_error = format!("{}", e);
                            Err(DuckyError::new("Delay value MUST be a number!", Some(err_data.as_str()), (verbose_error.as_str(), args.verbose)))
                        }
                    }?;
                    vec![[100, 0, delay_value, 0, 0, 0, 0, 0]]
                } 
            },
            "DEFAULT_DELAY" => { 
                let delay_value: KeyCode = match payload_tokens[1].to_string().parse() {
                    Ok(t) => Ok(t),
                    Err(e) => {
                        let verbose_error = format!("{}", e);
                        Err(DuckyError::new("Delay value MUST be a number!", Some(err_data.as_str()), (verbose_error.as_str(), args.verbose)))
                    }
                }?;
                vec![[200, 0, delay_value, 0, 0, 0, 0, 0]] 
            },
            _ => {
                let mut tmp_buf: Vec<KeyReport> = Vec::new();
                for token in payload_tokens {
                    tmp_buf.push(token.to_report(&kb_layout)?);
                }
                tmp_buf.push(RELEASE);
                tmp_buf
            }
        };
        for i in tmp_report { report_buf.push(i) }
    }
    Ok(report_buf)
}
