use spirv_headers::{Op, Word};
use spirv_tools::binary::{Parser, ParserOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the SPIR-V binary file
    let mut file = std::fs::File::open("shader.spv")?;

    // Parse the binary using spirv-tools
    let mut options = ParserOptions::new();
    options.validate = true; // enable validation
    let mut parser = Parser::new(&mut file, None, options)?;
    let header = parser.parse_header()?;
    let mut instr = parser.next()?;
    while let Some(word) = instr {
        // Parse the instruction
        let opcode = Op::try_from(word.to_le_bytes()[0] as u32).unwrap();
        let operands: Vec<Word> = word.to_le_bytes()[1..].chunks_exact(4).map(|c| Word::from_le_bytes(c.try_into().unwrap())).collect();
        // do something with the instruction
        println!("{} {:?}", opcode, operands);
        // get the next instruction
        instr = parser.next()?;
    }

    Ok(())
}
