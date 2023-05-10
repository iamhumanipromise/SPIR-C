extern crate shader_ripper;

use std::env;
use std::path::Path;
use shader_ripper::{rip_shader, generate_code};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <shader_file_path>", args[0]);
        return;
    }

    let file_path = Path::new(&args[1]);

    if let Some(extension) = file_path.extension() {
        if extension == "vert" || extension == "frag" {
            let shader_code = rip_shader(file_path);
            generate_code(shader_code);
            println!("Done!");
        } else {
            println!("Unsupported file type.");
        }
    } else {
        println!("Invalid file path.");
    }
}
