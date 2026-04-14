//! runtime_tauri crate: Tauri runtime command bridge.
//!
//! Provides #[tauri::command] handlers that bridge Tauri frontend
//! to usecases layer. Native-tauri imports commands from here.

pub mod commands;

/// Client-local schema bootstrapping for the embedded desktop runtime.
pub mod schema {
    /// SQL migration for the local counter table used by desktop runtime.
    pub const COUNTER_MIGRATION: &str = "CREATE TABLE IF NOT EXISTS counter (\
        tenant_id TEXT PRIMARY KEY,\
        value INTEGER NOT NULL DEFAULT 0,\
        updated_at TEXT NOT NULL DEFAULT (datetime('now'))\
    )";

    /// SQL migrations for local agent conversation storage used by desktop runtime.
    pub const AGENT_MIGRATIONS: &[&str] = &[
        "CREATE TABLE IF NOT EXISTS conversations (id TEXT PRIMARY KEY, title TEXT NOT NULL, created_at TEXT NOT NULL DEFAULT (datetime('now')))",
        "CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, conversation_id TEXT NOT NULL REFERENCES conversations(id), role TEXT NOT NULL, content TEXT NOT NULL, tool_calls TEXT, created_at TEXT NOT NULL DEFAULT (datetime('now')))",
    ];
}
