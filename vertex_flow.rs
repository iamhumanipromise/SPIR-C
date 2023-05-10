use regex::Regex;

pub struct VertexFlow {
    pub input: String,
    pub output: String,
}

impl VertexFlow {
    pub fn new(input: &str) -> Result<VertexFlow, Box<dyn std::error::Error>> {
        let re = Regex::new(r#"(?m)^ *in vec3 position;\n *in vec3 normal;\n *in vec2 texcoord;\n *out vec2 v_texcoord;\n *out vec3 v_normal;\n *void main\(\) {\n *gl_Position = vec4\(position, 1.0\);\n *v_texcoord = texcoord;\n *v_normal = normal;\n *}"#)?;

        let output = re.replace_all(input, "void main() { gl_Position = vec4(position, 1.0); }").to_string();

        Ok(VertexFlow {
            input: input.to_string(),
            output: output,
        })
    }
}
