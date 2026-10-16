use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub done: bool,
}

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, String> {
        let url = format!("{}/api/chat", self.base_url);
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
        };

        let res = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Request failed with status: {}", res.status()));
        }

        let response_body: ChatResponse = res.json().await
            .map_err(|e| e.to_string())?;
        Ok(response_body.message.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "hello".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"role":"user","content":"hello"}"#);
    }

    #[test]
    fn test_chat_message_deserialization() {
        let json = r#"{"role":"assistant","content":"Hi there!"}"#;
        let msg: ChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.role, "assistant");
        assert_eq!(msg.content, "Hi there!");
    }

    #[test]
    fn test_chat_request_serialization() {
        let req = ChatRequest {
            model: "llama3".to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello!".to_string(),
                },
            ],
            stream: false,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""model":"llama3""#));
        assert!(json.contains(r#""stream":false"#));
        assert!(json.contains(r#""role":"user""#));
        assert!(json.contains(r#""content":"Hello!""#));
    }

    #[test]
    fn test_chat_response_deserialization() {
        let json = r#"{
            "message": {
                "role": "assistant",
                "content": "Hello! How can I help you today?"
            },
            "done": true
        }"#;
        let res: ChatResponse = serde_json::from_str(json).unwrap();
        assert!(res.done);
        assert_eq!(res.message.role, "assistant");
        assert_eq!(res.message.content, "Hello! How can I help you today?");
    }

    #[test]
    fn test_ollama_client_new() {
        let client = OllamaClient::new("http://localhost:11434".to_string());
        assert_eq!(client.base_url, "http://localhost:11434");
    }
}
