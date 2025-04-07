use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use ollama_rs::{
    coordinator::Coordinator,
    generation::{chat::ChatMessage, parameters::JsonSchema, tools::Tool},
    Ollama,
};
use serde::Deserialize;

const MODEL_NAME: &str = "llama3.2:1b";

struct RandomNumberGeneratorTool;

#[derive(Deserialize, JsonSchema)]
struct RandomNumberGeneratorToolParams {
    #[schemars(description = "the minimum integer which can be generated")]
    min: usize,
    #[schemars(description = "the maximum integer which can be generated")]
    max: usize,
}

impl Tool for RandomNumberGeneratorTool {
    type Params = RandomNumberGeneratorToolParams;

    fn name() -> &'static str {
        "random_number_generator"
    }

    fn description() -> &'static str {
        "Generates a random integer between min and max (inclusive)"
    }

    async fn call(
        &mut self,
        params: Self::Params,
    ) -> Result<String, Box<(dyn Error + Send + Sync)>> {
        log::debug!("min = {}, max = {}", params.min, params.max);
        Ok(String::from("1"))
    }
}

#[tokio::main]
async fn main() {
    // env_logger::init();

    let history = vec![];

    let ollama = Ollama::default();
    let mut coordinator = Coordinator::new_with_tools(
        ollama,
        String::from(MODEL_NAME),
        history,
        RandomNumberGeneratorTool,
    );

    let stdin = stdin();

    loop {
        print!("-> ");

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim_end();

        if input.eq_ignore_ascii_case("bye") {
            break;
        }

        let res = coordinator
            .chat(vec![ChatMessage::user(input.to_string())])
            .await
            .unwrap();

        print!("<- {}", res.message.content);
    }
}
