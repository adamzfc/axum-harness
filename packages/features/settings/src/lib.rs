//! Settings feature — user preferences, theme, and privacy configuration.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// User settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub user_id: String,
    pub theme: Theme,
    pub language: String,
    pub notifications_enabled: bool,
    pub privacy: PrivacySettings,
}

/// UI theme preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

/// Privacy settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visible: bool,
    pub activity_visible: bool,
    pub data_collection_consent: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visible: true,
            activity_visible: true,
            data_collection_consent: false,
        }
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            theme: Theme::default(),
            language: "en".to_string(),
            notifications_enabled: true,
            privacy: PrivacySettings::default(),
        }
    }
}

/// Settings service trait.
#[async_trait]
pub trait SettingsService: Send + Sync {
    /// Get settings for a user.
    async fn get_settings(&self, user_id: &str) -> Result<UserSettings, SettingsError>;

    /// Update all settings.
    async fn update_settings(
        &self,
        user_id: &str,
        settings: UserSettings,
    ) -> Result<UserSettings, SettingsError>;

    /// Update a single setting (e.g., theme).
    async fn update_theme(&self, user_id: &str, theme: Theme) -> Result<(), SettingsError>;

    /// Reset settings to defaults.
    async fn reset_settings(&self, user_id: &str) -> Result<UserSettings, SettingsError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Settings not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
}
