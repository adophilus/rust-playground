use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use ollama_test::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    Result,
    gen::stream_response
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language?".to_string();

    let gen_req = GenerationRequest::new(model, prompt.clone()).system(DEFAULT_SYSTEM_MOCK.to_string());

    println!("->> prompt: {}", prompt);
    stream_response(&ollama, gen_req).await?;

    Ok(())
}
