use std::fmt::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use anyhow::anyhow;
use futures::{Stream, StreamExt};
use makepad_widgets::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// OpenAI API compatible LLM client
#[derive(Clone)]
pub struct LlmClient {
    client: Client,
    pub api_base: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ApiMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: DeltaContent,
}

#[derive(Deserialize)]
struct DeltaContent {
    content: Option<String>,
}

impl LlmClient {
    pub fn new(api_base: String, api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_base,
            api_key,
            model,
        }
    }

    /// Send a chat message and get streaming response
    pub async fn send_message_streaming(
        &self,
        messages: &[ChatMessage],
    ) -> Result<LlmStream, anyhow::Error> {
        let api_messages: Vec<ApiMessage> = messages
            .iter()
            .map(|m| ApiMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: api_messages,
            stream: true,
        };

        let url = format!("{}/chat/completions", self.api_base);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(error_text));
        }

        let stream = response.bytes_stream();
        let sse_stream = stream.filter_map(|result| async move {
            match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let lines: Vec<&str> = text.lines().collect();
                    let mut content = String::new();

                    for line in lines {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" {
                                continue;
                            }

                            if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
                                if let Some(choice) = response.choices.first() {
                                    if let Some(text) = &choice.delta.content {
                                        content.push_str(text);
                                    }
                                }
                            }
                        }
                    }

                    if content.is_empty() {
                        None
                    } else {
                        Some(Ok(content))
                    }
                }
                Err(e) => Some(Err(e)),
            }
        });

        Ok(LlmStream {
            inner: Box::pin(sse_stream),
        })
    }

    /// Send a chat message and get complete response (non-streaming)
    pub async fn send_message(&self, messages: &[ChatMessage]) -> Result<String, anyhow::Error> {
        let api_messages: Vec<ApiMessage> = messages
            .iter()
            .map(|m| ApiMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: api_messages,
            stream: false,
        };

        let url = format!("{}/chat/completions", self.api_base);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(format!("API error: {} - {}", status, error_text)));
        }

        let response_text = response.text().await?;

        // Parse the JSON response to extract content
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                if let Some(first) = choices.first() {
                    if let Some(content) = first
                        .get("message")
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                    {
                        return Ok(content.to_string());
                    }
                }
            }
        }

        Ok(response_text)
    }
}

/// Streaming response from LLM
pub struct LlmStream {
    inner: Pin<Box<dyn Stream<Item = Result<String, reqwest::Error>> + Send>>,
}

impl Stream for LlmStream {
    type Item = Result<String, reqwest::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_next_unpin(cx)
    }
}

/// Actions for LLM responses
#[derive(Debug, Clone)]
pub enum LlmAction {
    ResponseChunk(String),
    ResponseComplete,
    Error(String),
}

/// Default API configuration
pub const DEFAULT_API_BASE: &str = "https://api.openai.com/v1";
pub const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
