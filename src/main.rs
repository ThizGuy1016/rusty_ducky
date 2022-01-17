// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
//mod transpiler_functions;
//use transpiler_functions::parse_payload;
use std::time::Instant;

mod transpiler;
use transpiler::{Args, micro_ducky_transpiler::transpile, parser::Parser};

mod errors;
use errors::DuckyError;

fn main() -> Result<(), DuckyError> {
    
    let now = Instant::now();
    let args: Args = Args::new();

    {

        let parser = Parser::new(&args)?;
        let parser_tokens = parser.parse_payload()?;

        transpile(&args, &parser_tokens)?;
        
        //if args.transpile { transpile(&args, &report_buf)? }
        //if args.compile { todo!() }
    
    }

    let elapsed = now.elapsed();
    let output_size = std::fs::metadata(&args.output_file)?.len();
    let mut display_size = format!("{}B", output_size);
    if output_size > 1_000 { display_size = format!("{:.2}K", output_size/1000)}
    else if output_size > 1_000_000 { display_size = format!("{:.2}M", output_size/1_000_000)}
    else if output_size > 1_000_000_000 { display_size = format!("{:.2}G", output_size/1_000_000_000)}

    println!("[SUCCESS] Tranpiled with no errors.\n| Input: '{}'\n| Output: '{}'\n| Output size: '{}'\n| Layout: '{}'\n| Time elapsed: {:.2?}", &args.payload_file, &args.output_file, display_size, &args.keyboard_language, elapsed);

    if output_size > 64_000 { println!("\n[WARNING] Output file size.\n| Due to the limited ammount of RAM on certain microcontrollers,\n| files exceeding 64K in size might not fit.\n| Check your board's specifications to make sure the payload will run.")}

    Ok(())

}
