use std::fs;
use std::io;
use std::path::PathBuf;
use clap::{Arg, App};

// Define our CLI arguments
fn define_cli() -> App<'static, 'static> {
    App::new("SPIR-C")
        .version("0.1.0")
        .author("Your Name <yourname@example.com>")
        .about("Compiles GLSL to SPIR-V bytecode")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .value_name("FILE")
             .help("Input GLSL file to compile")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE")
             .help("Output SPIR-V file")
             .takes_value(true)
             .required(true))
}

fn main() -> io::Result<()> {
    let matches = define_cli().get_matches();

    // Get input and output file paths
    let input_path = PathBuf::from(matches.value_of("input").unwrap());
    let output_path = PathBuf::from(matches.value_of("output").unwrap());

    // Read input file
    let input_str = fs::read_to_string(&input_path)?;

    // Call the SPIR-C compiler
    let spirv_code = spir_c::compile(&input_str);

    // Write output file
    fs::write(output_path, spirv_code)?;

    Ok(())
}
