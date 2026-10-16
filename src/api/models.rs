use serde::{Deserialize, Serialize};

/// Represents a single message in a conversation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    /// Creates a system role message.
    #[allow(dead_code)]
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    /// Creates a user role message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    /// Creates an assistant role message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

/// Request payload sent to Ollama's `/api/chat` endpoint.
#[derive(Serialize, Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

/// Response payload received from Ollama's `/api/chat` endpoint.
#[derive(Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub message: ChatMessage,
    #[allow(dead_code)]
    pub done: bool,
}

/// Information about a single local model from Ollama.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelInfo {
    pub name: String,
    #[serde(default)]
    pub model: String,
}

/// Response payload received from Ollama's `/api/tags` endpoint.
#[derive(Deserialize, Debug, Clone)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_helpers() {
        let s = ChatMessage::system("You are helpful.");
        assert_eq!(s.role, "system");
        assert_eq!(s.content, "You are helpful.");

        let u = ChatMessage::user("Hello");
        assert_eq!(u.role, "user");
        assert_eq!(u.content, "Hello");

        let a = ChatMessage::assistant("Hi!");
        assert_eq!(a.role, "assistant");
        assert_eq!(a.content, "Hi!");
    }

    #[test]
    fn test_chat_request_serialization() {
        let req = ChatRequest {
            model: "llama3".to_string(),
            messages: vec![ChatMessage::user("Test")],
            stream: false,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""model":"llama3""#));
        assert!(json.contains(r#""stream":false"#));
    }

    #[test]
    fn test_chat_response_deserialization() {
        let json = r#"{
            "message": {
                "role": "assistant",
                "content": "Response"
            },
            "done": true
        }"#;
        let res: ChatResponse = serde_json::from_str(json).unwrap();
        assert!(res.done);
        assert_eq!(res.message.content, "Response");
    }

    #[test]
    fn test_list_models_response_deserialization() {
        let json = r#"{
            "models": [
                {
                    "name": "llama3:latest",
                    "model": "llama3:latest"
                },
                {
                    "name": "mistral:latest",
                    "model": "mistral:latest"
                }
            ]
        }"#;
        let res: ListModelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res.models.len(), 2);
        assert_eq!(res.models[0].name, "llama3:latest");
        assert_eq!(res.models[1].name, "mistral:latest");
    }
}
