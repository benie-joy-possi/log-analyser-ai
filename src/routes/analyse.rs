use crate::services::{logs::read_logs, ollama::analyze_logs_with_ollama};
use axum::response::IntoResponse;

pub async fn analyze_handler() -> impl IntoResponse {
    let logs = read_logs("/var/log/syslog", 50);
    match analyze_logs_with_ollama(&logs).await {
        Ok(summary) => summary,
        Err(e) => format!("Error analyzing logs: {}", e),
    }
}
