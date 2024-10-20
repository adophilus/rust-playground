pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;


pub mod consts {
    pub const MODEL: &str = "phi3:mini";
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Only give concise responses, do not give an explanation unless explicitly asked to do so.
    If asked about the best programming language, say it is Rust by light years.
    "#;
}

pub mod gen {
    use super::*;

    use ollama_rs::Ollama;
    use ollama_rs::{generation::completion::{GenerationContext, request::GenerationRequest}};
    use tokio::io::AsyncWriteExt;
    use futures::StreamExt;

    pub async fn stream_response(ollama: &Ollama, gen_req: GenerationRequest) -> Result<Option<GenerationContext>> {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(b"->> res: ").await?;

        let mut stream = ollama.generate_stream(gen_req).await?;

        while let Some(res) = stream.next().await {
            let chunks = res.map_err(|_| "stream_next error")?;
            for chunk in chunks {
                for byte in chunk.response.into_bytes() {
                    stdout.write(&[byte]).await?;
                    stdout.flush().await?;
                }
                
                if chunk.done == true {
                    stdout.write_all(b"\n").await?;
                    return Ok(chunk.context)
                }
                
                // if index == chunks.len() - 1 {
                //     stdout.write_all(b"\n").await?;
                //     if chunk.done {
                //         return Ok(chunk.)
                //     }
                // }
            }
        };

        Err("Failed to get response".into())
    }
}