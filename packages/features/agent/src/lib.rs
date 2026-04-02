//! Agent feature — chat with tool calling and streaming.

use async_trait::async_trait;
use contracts_api::ChatMessage;
use serde::{Deserialize, Serialize};

/// Conversation summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: String,
}

/// Agent service trait.
#[async_trait]
pub trait AgentService: Send + Sync {
    async fn create_conversation(&self, title: &str) -> Result<Conversation, AgentError>;
    async fn get_conversations(&self) -> Result<Vec<Conversation>, AgentError>;
    async fn get_messages(&self, conversation_id: &str) -> Result<Vec<ChatMessage>, AgentError>;
    async fn send_message(
        &self,
        conversation_id: &str,
        content: &str,
    ) -> Result<ChatMessage, AgentError>;
    async fn chat_stream(
        &self,
        conversation_id: &str,
        content: &str,
        api_key: &str,
        base_url: &str,
        model: &str,
    ) -> Result<
        std::pin::Pin<Box<dyn futures_util::Stream<Item = Result<String, AgentError>> + Send>>,
        AgentError,
    >;
}

/// Read-only tool definitions.
pub const AVAILABLE_TOOLS: &[(&str, &str)] = &[
    ("get_counter_value", "Get the current counter value"),
    ("list_tenants", "List all tenants"),
    ("get_system_status", "Get system health status"),
];

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Database error: {0}")]
    Database(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("API error: {0}")]
    Api(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Conversation not found: {0}")]
    NotFound(String),
}
