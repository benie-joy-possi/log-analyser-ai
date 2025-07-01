use crate::models::ollama::{OllamaRequest, OllamaResponse};
use reqwest::Client;

pub async fn analyze_logs_with_ollama(logs: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let body = OllamaRequest {
        model: "llama3.2:latest",
        prompt: &format!("Analyze and summarize the following logs:\n{}", logs),
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await?;

    let parsed: OllamaResponse = res.json().await?;
    Ok(parsed.response)
}
