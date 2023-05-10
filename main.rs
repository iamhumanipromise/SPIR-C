use std::fs::File;
use std::io::prelude::*;

mod vertex_flow;
mod shader_ripper;
mod spirv_optimizer;
mod glsl_to_spirv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.glsl")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let vertex_flow = vertex_flow::VertexFlow::new(&contents)?;
    let spirv = glsl_to_spirv::compile(&contents)?;
    let spirv_optimized = spirv_optimizer::optimize(&spirv)?;
    let spirv_as_bytes = spirv_optimized.as_binary_u8();

    shader_ripper::rip_shader(&spirv_as_bytes)?;

    Ok(())
}
