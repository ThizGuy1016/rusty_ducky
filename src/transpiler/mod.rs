// main should be able to compile or transpile
pub mod rusty_ducky_compiler;
pub mod micro_ducky_transpiler;
pub mod parser;

pub use micro_ducky_transpiler::transpile;
pub use parser::parse_payload;
//pub use rusty_ducky_compiler::compile;

pub type KeyCode = u16;
pub type KeyReport = [KeyCode; 8];
pub const RELEASE: KeyReport = [0, 0, 0, 0, 0, 0, 0, 0];

extern crate clap;

use clap::{App, Arg};
use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub struct Args {
    payload_file: String,
    template_file: Option<String>,
    output_file: String,
    keyboard_language: String,
    pub compile: bool,
    pub transpile: bool,
    verbose: bool
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
            .version("0.4")
            .about("A Rust transpiler and compiler for keystroke injections on imbedded devices.")
            .author("Carter Vavra");

        let payload_file_option = Arg::new("payload")
            .long("payload")
            .short('p')
            .takes_value(true)
            .help("Points rusty ducky to a payload file to transpile.\nDefault is payload.txt")
            .default_value("payload.txt")
            .required(false);
        let app = app.arg(payload_file_option);

        let template_file_option = Arg::new("template")
        .long("template")
        .short('i')
        .takes_value(true)
        .help("Overrides rusty ducky's template circuit python file to a file of your choosing.")
        .required(false);
        let app = app.arg(template_file_option);

        let output_file_option = Arg::new("output")
        .long("output")
        .short('o')
        .takes_value(true)
        .help("Specify a name for the transpiled cuircut python file.")
        .default_value("Code.py")
        .required(false);
        let app = app.arg(output_file_option);

        let keyboard_language_option = Arg::new("language")
        .long("language")
        .short('l')
        .takes_value(true)
        .help("Points rusty ducky to a [keyboard_language].json file to parse.")
        .default_value("US")
        .required(false);
        let app = app.arg(keyboard_language_option);

        let compile_option = Arg::new("compile")
        .long("compile")
        .short('c')
        .takes_value(false)
        .help("Tells rusty ducky to compile the payload file. [NOT IMPLEMENTED YET!]")
        .required(false);
        let app = app.arg(compile_option);

        let transpile_option = Arg::new("transpile")
        .long("transpile")
        .short('t')
        .takes_value(false)
        .help("Tells rusty ducky to transpile the payload file to cuircut python.")
        .required(false);
        let app = app.arg(transpile_option);

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
        let compiled = matches.is_present("compile");
        let transpiled = matches.is_present("transpile");

        let mut template: Option<String> = None;
        if matches.is_present("template") { template = Some(matches.value_of("template").unwrap().to_string())}

        Ok( Args{ 
            payload_file: payload_file.to_string(),
            output_file: output_file.to_string(),
            template_file: template,
            keyboard_language: keyboard_language.to_string(),
            compile: compiled,
            transpile: transpiled,
            verbose: verbose
        } )
    }
}