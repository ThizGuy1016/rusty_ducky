pub mod circuit_transpiler;
pub mod parser;
pub mod ducky_io;

pub use parser::Parser;
pub use circuit_transpiler::transpile;
pub use ducky_io::*;

pub type KeyValue = u16;
pub type KeyReport = [KeyValue; 8];
pub const RELEASE: KeyReport = [0, 0, 0, 0, 0, 0, 0, 0];

extern crate clap;

use clap::{App, Arg};
use std::ffi::OsString;
use once_cell::sync::Lazy;

pub static ARGS: Lazy<Args> = Lazy::new(||{Args::new()});

#[derive(Debug, PartialEq, Clone)]
pub struct Args {
    pub payload_file: String,
    template_file: Option<String>,
    pub output_file: String,
    pub keyboard_language: String,
    pub verbose: bool
}

impl Args {
    pub fn new() -> Args {
        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    fn new_from<I, T>(args: I) -> Result<Args, clap::Error>
    where 
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,    
    {
        let app = App::new("rusty_ducky")
            .version("0.5")
            .about("rusty_ducky is a DuckyScript keystroke injection toolkit for microcontrollers that support Circuit Python.")
            .author("ITDude");

        let payload_file_option = Arg::new("payload")
            .long("payload")
            .short('p')
            .takes_value(true)
            .help("Points rusty ducky to a payload file to transpile.")
            .default_value("payload.txt")
            .required(false);
        let app = app.arg(payload_file_option);

        let template_file_option = Arg::new("template")
        .long("template")
        .short('t')
        .takes_value(true)
        .help("Overrides rusty ducky's template circuit python file to a file of your choosing.")
        .required(false);
        let app = app.arg(template_file_option);

        let output_file_option = Arg::new("output")
        .long("output")
        .short('o')
        .takes_value(true)
        .help("Specify a name for the transpiled circuit python file.")
        .default_value("Code.py")
        .required(false);
        let app = app.arg(output_file_option);

        let keyboard_language_option = Arg::new("language")
        .long("language")
        .short('l')
        .takes_value(true)
        .help("Points rusty ducky to a [keyboard_language].json file to parse.")
        .default_value("US.json")
        .required(false);
        let app = app.arg(keyboard_language_option);

        let verbose_option = Arg::new("verbose")
        .long("verbose")
        .short('v')
        .takes_value(false)
        .help("Sets the verbosity level of rusty ducky errors.")
        .required(false);
        let app = app.arg(verbose_option);

        let matches = app.try_get_matches_from(args)?;

        let payload_file = matches
            .value_of("payload")
            .unwrap();
        let output_file = matches
            .value_of("output")
            .unwrap();
        let keyboard_language = matches
            .value_of("language")
            .unwrap();
        let verbose = matches.is_present("verbose");

        let mut template: Option<String> = None;
        if matches.is_present("template") { template = Some(matches.value_of("template").unwrap().to_string())}

        Ok( Args{ 
            payload_file: payload_file.to_string(),
            output_file: output_file.to_string(),
            template_file: template,
            keyboard_language: keyboard_language.to_string(),
            verbose: verbose
        } )
    }
}