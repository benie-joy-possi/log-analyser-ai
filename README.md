# 🧠 AIOps Log Analyzer with Rust + LLaMA 3 (Ollama)

This is a local AIOps tool that reads system logs (e.g., `/var/log/syslog`) and uses a locally hosted LLaMA 3 model via [Ollama](https://ollama.com) to analyze, summarize, and explain any anomalies or failures in natural language.

Built with:

- 🚀 Rust (Axum + Tokio)
- 🤖 LLaMA 3.2:latest via Ollama
- 📄 Linux system logs
- 🔗 Async API calls (Reqwest)

---

## 📌 Features

- ✅ Read last N lines from system log files
- ✅ Analyze log content using a local LLM
- ✅ Serve results over a REST API: `GET /analyze`
- ✅ 100% private & offline — no cloud dependency

---

## 📁 Project Structure
```bash
log-analyzer-ai/
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── routes/
│ │ └── mod.rs # declares analyze module
│ │ └── analyze.rs # analyze route handler
│ ├── services/
│ │ └── mod.rs # declares logs and ollama modules
│ │ ├── logs.rs # log reading utilities
│ │ └── ollama.rs # Ollama API integration
│ └── models/
│ └── mod.rs # declares ollama model structs
│ └── ollama.rs # request/response structs for Ollama
├── README.md
└── .env (optional)
```

---

## ⚙️ Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Ollama](https://ollama.com/) installed and running
- `llama3.2:latest` model pulled locally via:

```bash
ollama pull llama3.2:latest
```

## 🛠️ Running the App

### 1. Clone the repo

```bash
git clone <your-repo-url>
cd log-analyzer-ai
```

### 2. Run the Ollama model

In a **separate terminal**, start the LLaMA 3 model:

```bash
ollama run llama3.2:latest
```

### 3. Run the server

In your **main terminal**, start the Axum server:

```bash
cargo run
```

By default, the server starts on:

```bash
http://localhost:3000/analyze
```

## 🧪 Test the Endpoint

Use **curl** or a **browser** to make a request:

```bash
curl http://localhost:3000/analyze
```

You’ll receive a **natural-language explanation** of the last 50 lines of **/var/log/syslog**.
