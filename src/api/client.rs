use super::models::{ChatMessage, ChatRequest, ChatResponse};
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
            ApiError::HttpStatus(code) => write!(f, "HTTP error with status code: {}", code),
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
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, ApiError> {
        let url = format!("{}/api/chat", self.base_url);
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
        };

        let res = self.client.post(&url)
            .json(&request)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(ApiError::HttpStatus(res.status().as_u16()));
        }

        let response_body: ChatResponse = res.json().await?;
        Ok(response_body.message.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = OllamaClient::new("http://localhost:11434");
        assert_eq!(client.base_url(), "http://localhost:11434");
    }

    #[test]
    fn test_error_display() {
        let err = ApiError::HttpStatus(404);
        assert_eq!(format!("{}", err), "HTTP error with status code: 404");
    }
}
