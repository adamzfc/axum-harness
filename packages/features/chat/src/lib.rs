//! Chat feature — real-time messaging with WebSocket/SSE push.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub sender_id: String,
    pub content: String,
    pub created_at: String,
}

/// Chat session (conversation thread).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub participants: Vec<String>,
    pub last_message_at: String,
    pub message_count: u64,
}

/// Chat service trait — defines chat operations.
#[async_trait]
pub trait ChatService: Send + Sync {
    /// Create a new chat session.
    async fn create_session(&self, participants: &[String]) -> Result<ChatSession, ChatError>;

    /// List all sessions for a user.
    async fn list_sessions(&self, user_id: &str) -> Result<Vec<ChatSession>, ChatError>;

    /// Get messages in a session (paginated).
    async fn get_messages(
        &self,
        session_id: &str,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<ChatMessage>, ChatError>;

    /// Send a message to a session.
    async fn send_message(
        &self,
        session_id: &str,
        sender_id: &str,
        content: &str,
    ) -> Result<ChatMessage, ChatError>;

    /// Subscribe to real-time message stream for a session.
    /// Returns a stream of messages as they arrive.
    async fn subscribe(
        &self,
        session_id: &str,
    ) -> Result<
        std::pin::Pin<Box<dyn futures_util::Stream<Item = Result<ChatMessage, ChatError>> + Send>>,
        ChatError,
    >;
}

#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Session not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Stream error: {0}")]
    StreamError(String),
}
