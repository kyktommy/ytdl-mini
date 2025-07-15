use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Download status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Success,
    Failed(String),
}

/// Individual download item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub status: DownloadStatus,
    pub created_at: DateTime<Utc>,
    pub progress: f32, // 0.0 to 1.0
    pub file_path: Option<String>,
}

impl DownloadItem {
    pub fn new(url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            url,
            title: None,
            status: DownloadStatus::Pending,
            created_at: Utc::now(),
            progress: 0.0,
            file_path: None,
        }
    }
}

/// Download manager handles the queue and processing of downloads
pub struct DownloadManager {
    downloads: HashMap<Uuid, DownloadItem>,
    active_downloads: usize,
    max_concurrent: usize,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            downloads: HashMap::new(),
            active_downloads: 0,
            max_concurrent: 3,
        }
    }

    /// Add a new download to the queue
    pub async fn add_download(&mut self, url: String) -> Result<Uuid, anyhow::Error> {
        // Validate URL first
        if !crate::utils::is_valid_youtube_url(&url) {
            return Err(anyhow::anyhow!("Invalid YouTube URL"));
        }

        let download_item = DownloadItem::new(url);
        let id = download_item.id;
        
        self.downloads.insert(id, download_item);
        
        // Try to start download if we have capacity
        self.try_start_next_download().await;
        
        Ok(id)
    }

    /// Get all downloads
    pub fn get_downloads(&self) -> Vec<DownloadItem> {
        let mut downloads: Vec<_> = self.downloads.values().cloned().collect();
        downloads.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        downloads
    }

    /// Update download status
    pub fn update_download_status(&mut self, id: Uuid, status: DownloadStatus) {
        if let Some(download) = self.downloads.get_mut(&id) {
            download.status = status;
            
            // If download finished (success or failed), decrement active count
            if matches!(download.status, DownloadStatus::Success | DownloadStatus::Failed(_)) {
                self.active_downloads = self.active_downloads.saturating_sub(1);
            }
        }
    }

    /// Update download progress
    pub fn update_download_progress(&mut self, id: Uuid, progress: f32) {
        if let Some(download) = self.downloads.get_mut(&id) {
            download.progress = progress.clamp(0.0, 1.0);
        }
    }

    /// Update download title
    pub fn update_download_title(&mut self, id: Uuid, title: String) {
        if let Some(download) = self.downloads.get_mut(&id) {
            download.title = Some(title);
        }
    }

    /// Try to start the next pending download
    async fn try_start_next_download(&mut self) {
        if self.active_downloads >= self.max_concurrent {
            return;
        }

        // Find next pending download
        let next_download = self.downloads
            .values()
            .find(|d| d.status == DownloadStatus::Pending)
            .map(|d| d.id);

        if let Some(id) = next_download {
            self.start_download(id).await;
        }
    }

    /// Start a specific download
    async fn start_download(&mut self, id: Uuid) {
        if let Some(download) = self.downloads.get_mut(&id) {
            download.status = DownloadStatus::Downloading;
            self.active_downloads += 1;

            // Clone necessary data for the async task
            let url = download.url.clone();
            let download_id = id;

            // Spawn download task
            tokio::spawn(async move {
                // This is a placeholder for the actual download logic
                // In a real implementation, this would:
                // 1. Initialize YT-DLP
                // 2. Get video metadata
                // 3. Start the download
                // 4. Update progress
                // 5. Handle completion/errors

                log::info!("Starting download for: {}", url);

                // Simulate download process
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                // TODO: Integrate with actual YT-DLP download logic
                // For now, we'll just mark it as successful
                log::info!("Download completed for: {}", url);
            });
        }
    }

    /// Set maximum concurrent downloads
    pub fn set_max_concurrent(&mut self, max: usize) {
        self.max_concurrent = max.max(1); // Ensure at least 1
    }

    /// Remove a download
    pub fn remove_download(&mut self, id: Uuid) -> Option<DownloadItem> {
        self.downloads.remove(&id)
    }

    /// Clear all completed downloads
    pub fn clear_completed(&mut self) {
        self.downloads.retain(|_, download| {
            !matches!(download.status, DownloadStatus::Success)
        });
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
