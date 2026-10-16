use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub done: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_helpers() {
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
}
