// parses the provided duckyscript payload into tokens that can be used for compilation, transpilation, or even execution
//mod transpiler_functions;
//use transpiler_functions::parse_payload;
mod transpiler;
use transpiler::{Args, micro_ducky_transpiler::transpile, parser::parse_payload};

mod errors;
use errors::DuckyError;

fn main() -> Result<(), DuckyError> {
    let args: Args = Args::new();

    let report_buf = parse_payload(&args)?;

    if args.transpile { transpile(&args, &report_buf)? }
    //if args.compile { todo!() }

    Ok(())

}
