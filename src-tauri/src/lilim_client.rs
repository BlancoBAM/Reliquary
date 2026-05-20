/// lilim_client.rs — HTTP client for Lilim / Ollama-compatible inference server.
/// Replaces the ollama_rs crate with direct reqwest calls so Reliquary can
/// work with Lilim (https://github.com/BlancoBAM/Lilim) out-of-the-box while
/// remaining backward-compatible with a vanilla Ollama endpoint.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

// ── Request / Response shapes ────────────────────────────────────────────────

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Single-prompt legacy embedding (Ollama /api/embeddings format).
#[derive(Serialize)]
struct EmbedLegacyRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbedLegacyResponse {
    embedding: Vec<f32>,
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    pub response: String,
}

// ── Client ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LilimClient {
    pub base_url: String,
    client: reqwest::Client,
}

impl LilimClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Generate embeddings for a batch of text chunks.
    /// Tries the Ollama v0.4+ `/api/embed` endpoint first; falls back to the
    /// legacy single-prompt `/api/embeddings` if that fails (older Ollama /
    /// Lilim versions).
    pub async fn generate_embeddings(
        &self,
        model: &str,
        texts: Vec<&str>,
    ) -> Result<Vec<Vec<f32>>> {
        let batch_url = format!("{}/api/embed", self.base_url);
        let body = EmbedRequest { model, input: texts.clone() };

        let resp = self.client.post(&batch_url).json(&body).send().await;

        match resp {
            Ok(r) if r.status().is_success() => {
                let er: EmbedResponse = r.json().await?;
                return Ok(er.embeddings);
            }
            _ => {}
        }

        // Fallback: call /api/embeddings once per chunk and collect
        let legacy_url = format!("{}/api/embeddings", self.base_url);
        let mut all: Vec<Vec<f32>> = Vec::with_capacity(texts.len());
        for text in &texts {
            let body = EmbedLegacyRequest { model, prompt: text };
            let r = self
                .client
                .post(&legacy_url)
                .json(&body)
                .send()
                .await
                .map_err(|e| anyhow!("Embedding request failed: {e}"))?;
            let er: EmbedLegacyResponse = r
                .json()
                .await
                .map_err(|e| anyhow!("Failed to parse embedding response: {e}"))?;
            all.push(er.embedding);
        }
        Ok(all)
    }

    /// Run a text completion against the LLM.
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        let body = GenerateRequest { model, prompt, stream: false };
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Generate request failed: {e}"))?;
        let gr: GenerateResponse = resp
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse generate response: {e}"))?;
        Ok(gr.response)
    }

    /// Health-check: returns true if the server is reachable.
    pub async fn ping(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
