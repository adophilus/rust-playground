use crate::consts::{DEFAULT_SYSTEM_MOCK, MODEL};
use crate::Result;
use ollama_rs::Ollama;

#[tokio::main]
fn main() -> Result<()> {
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (Be consise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    let res = ollama.generate(gen_req).await?;
    println!("->> res: {}", res.response);

    Ok(())
}
