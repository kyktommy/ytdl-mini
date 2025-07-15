use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::{DownloadManager, Settings};

/// Main application state
#[derive(Clone)]
pub struct AppState {
    pub download_manager: Arc<RwLock<DownloadManager>>,
    pub settings: Arc<RwLock<Settings>>,
    pub current_url: Arc<RwLock<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            download_manager: Arc::new(RwLock::new(DownloadManager::new())),
            settings: Arc::new(RwLock::new(Settings::default())),
            current_url: Arc::new(RwLock::new(String::new())),
        }
    }

    /// Add a new download to the queue
    pub async fn add_download(&self, url: String) -> Result<Uuid, anyhow::Error> {
        let mut manager = self.download_manager.write().await;
        manager.add_download(url).await
    }

    /// Get all downloads
    pub async fn get_downloads(&self) -> Vec<super::DownloadItem> {
        let manager = self.download_manager.read().await;
        manager.get_downloads()
    }

    /// Update the current URL input
    pub async fn set_current_url(&self, url: String) {
        let mut current_url = self.current_url.write().await;
        *current_url = url;
    }

    /// Get the current URL input
    pub async fn get_current_url(&self) -> String {
        let current_url = self.current_url.read().await;
        current_url.clone()
    }

    /// Update settings
    pub async fn update_settings(&self, new_settings: Settings) {
        let mut settings = self.settings.write().await;
        *settings = new_settings;
    }

    /// Get current settings
    pub async fn get_settings(&self) -> Settings {
        let settings = self.settings.read().await;
        settings.clone()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
