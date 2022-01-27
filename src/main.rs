// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
extern crate colored;

mod transpiler;
use transpiler::{transpile, Parser, ARGS};

mod errors;
use errors::DuckyError;

use std::time::Instant;
use colored::*;

fn main() -> Result<(), DuckyError> {
    
    let now = Instant::now();

    {
        let parser = Parser::new()?;
        let payload_tokens = parser.parse_payload()?;

        transpile(payload_tokens)?;
    }

    let elapsed = now.elapsed();
    let output_size = std::fs::metadata(&ARGS.output_file)?.len();
    let mut display_size = format!("{}B", output_size);

    // changes filesize output based on output filesize
    if output_size > 1_000 { display_size = format!("{:.2}K", output_size/1000)}
    else if output_size > 1_000_000 { display_size = format!("{:.2}M", output_size/1_000_000)}
    else if output_size > 1_000_000_000 { display_size = format!("{:.2}G", output_size/1_000_000_000)}

    if output_size > 50_000 { 
        print!("\n{} Output file size.\n", "[WARNING]".yellow().bold());
        print!("| Due to the limited amount of FLASH storage on Non-Express microcontrollers,\n");
        print!("| Files exceeding {} in size might not fit.\n", "50K".yellow().bold());
        print!("| Check out Circuit Python's expectations page for more details.\n");
        print!("| {}", "https://learn.adafruit.com/circuitpython-essentials/circuitpython-expectations\n".green());
    }
    
    print!("{} {}\n", "[SUCCESS]".green().bold(), "Tranpiled with no errors.".white());
    print!("| Input: '{}'\n", &ARGS.payload_file.blue());
    print!("| Output: '{}'\n", &ARGS.output_file.blue());
    print!("| Output size: '{}'\n", display_size.blue());
    print!("| Layout: '{}'\n", &ARGS.keyboard_language.blue());
    print!("| Time elapsed: {:.2?}\n", elapsed);

    Ok(())

}
