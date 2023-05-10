use std::fs::File;
use std::io::prelude::*;

pub fn read_spirv_file(file_path: &str) -> Vec<u32> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut spirv_code = Vec::new();
    file.read_to_end(&mut spirv_code).expect("Failed to read file");
    let spirv_words = spirv_code.chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect::<Vec<u32>>();
    spirv_words
}

pub fn decode_spirv_code(spirv_code: &[u32]) -> Result<spirv_headers::Header, spirv_headers::Error> {
    let (_, header) = spirv_headers::decode_bytes(spirv_code)?;
    Ok(header)
}

pub fn print_spirv_header_info(header: &spirv_headers::Header) {
    println!("SPIR-V Version: {}.{}", header.version_major, header.version_minor);
    println!("Generator ID: {}", header.generator_id);
    println!("Bound: {}", header.bound);
}

pub fn optimize_spirv_code(spirv_code: &[u32], optimizer_options: &str) -> Result<Vec<u32>, std::io::Error> {
    let mut process = std::process::Command::new("spirv-opt")
        .args(&[optimizer_options, "-o", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let stdin = process.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(bytemuck::cast_slice(spirv_code)).expect("Failed to write to stdin");
    let output = process.wait_with_output()?;
    let optimized_spirv_code = output.stdout.chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect::<Vec<u32>>();
    Ok(optimized_spirv_code)
}

pub fn write_spirv_file(file_path: &str, spirv_code: &[u32]) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    for word in spirv_code {
        file.write_all(&word.to_le_bytes())?;
    }
    Ok(())
}
