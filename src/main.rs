// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
//mod transpiler_functions;
//use transpiler_functions::parse_payload;
mod transpiler;
use transpiler::{Args, micro_ducky_transpiler::transpile, parser::parse_payload};

mod errors;
use errors::DuckyError;


fn main() -> Result<(), DuckyError> {
    let args: Args = Args::new();

    if !args.transpile && !args.compile { 
        println!("[ERROR] Must provide a compilation type.\nRun rusty_ducky with the -h flag to see more info.\nEx: ./rusty_ducky -h");
        return Ok(());
    }

    let report_buf = match parse_payload(&args) {
        Ok(r) => r,
        Err(e) => {println!("{:?}", e); vec![[0, 0, 0, 0, 0, 0, 0, 0]]}
    };

    if args.transpile { transpile(&args, &report_buf)? }
    if args.compile { todo!() }

    Ok(())

}
