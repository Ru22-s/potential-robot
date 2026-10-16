use super::models::{ChatMessage, ChatRequest, ChatResponse, ListModelsResponse};
use reqwest::Client;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    HttpStatus(u16),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Network(e) => write!(f, "Network error: {}", e),
            ApiError::HttpStatus(code) => write!(f, "HTTP error code: {}", code),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Network(err)
    }
}

#[derive(Clone, Debug)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    /// Creates a new Ollama client with base URL (e.g., `http://localhost:11434`).
    pub fn new(base_url: impl Into<String>) -> Self {
        let base_url = base_url.into();
        let base_url = base_url.trim_end_matches('/').to_string();
        Self {
            client: Client::new(),
            base_url,
        }
    }

    #[allow(dead_code)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Sends a chat completion request to Ollama's `/api/chat`.
    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, ApiError> {
        let url = format!("{}/api/chat", self.base_url);
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
        };

        let res = self.client.post(&url).json(&request).send().await?;

        if !res.status().is_success() {
            return Err(ApiError::HttpStatus(res.status().as_u16()));
        }

        let response_body: ChatResponse = res.json().await?;
        Ok(response_body.message.content)
    }

    /// Fetches list of locally installed model names from Ollama's `/api/tags`.
    pub async fn list_models(&self) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/tags", self.base_url);
        let res = self.client.get(&url).send().await?;

        if !res.status().is_success() {
            return Err(ApiError::HttpStatus(res.status().as_u16()));
        }

        let response_body: ListModelsResponse = res.json().await?;
        Ok(response_body.models.into_iter().map(|m| m.name).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new_trims_trailing_slash() {
        let client1 = OllamaClient::new("http://localhost:11434");
        assert_eq!(client1.base_url(), "http://localhost:11434");

        let client2 = OllamaClient::new("http://localhost:11434/");
        assert_eq!(client2.base_url(), "http://localhost:11434");
    }

    #[test]
    fn test_error_display() {
        let err = ApiError::HttpStatus(404);
        assert_eq!(format!("{}", err), "HTTP error code: 404");
    }
}
