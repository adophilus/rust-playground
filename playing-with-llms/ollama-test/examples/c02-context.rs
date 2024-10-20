use ollama_rs::{generation::completion::{GenerationContext, request::GenerationRequest}, Ollama};
use ollama_test::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    Result,
    gen::stream_response
};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompts = &["Why is the sky red?", "What was my first question?"];

    let mut context: Option<GenerationContext> = None;

    for prompt in prompts {
        let mut gen_req = GenerationRequest::new(model.to_string(), prompt.to_string()).system(String::from("Always ensure your answers as as concise as possible. They should never exceed a 100 characters."));
        if let Some(ctx) = context.take() {
            gen_req = gen_req.context(ctx);
        }

        println!("->> prompt: {}", prompt);
        context = stream_response(&ollama, gen_req).await?;
        tokio::fs::File::create("context.json").await?
        .write_all(serde_json::json!(context.clone()).to_string().as_bytes()).await?;
    }


    Ok(())
}
