use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Default video resolution for downloads
    pub default_resolution: String,
    /// Download destination directory
    pub download_path: PathBuf,
    /// Maximum concurrent downloads
    pub max_concurrent_downloads: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_resolution: "1920x1080".to_string(),
            download_path: crate::utils::get_downloads_dir(),
            max_concurrent_downloads: 3,
        }
    }
}

impl Settings {
    /// Load settings from file
    pub fn load() -> Result<Self, anyhow::Error> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let settings: Settings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Self::default())
        }
    }

    /// Save settings to file
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        
        Ok(())
    }

    /// Get the configuration file path
    fn config_path() -> Result<PathBuf, anyhow::Error> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        Ok(config_dir.join("ytdl-mini").join("config.json"))
    }

    /// Validate and update resolution
    pub fn set_resolution(&mut self, resolution: String) -> Result<(), anyhow::Error> {
        // Basic validation for resolution format
        if resolution.contains('x') && resolution.split('x').count() == 2 {
            self.default_resolution = resolution;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid resolution format. Use format like '1920x1080'"))
        }
    }

    /// Set download path
    pub fn set_download_path(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {
        if path.exists() || std::fs::create_dir_all(&path).is_ok() {
            self.download_path = path;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Cannot create or access download directory"))
        }
    }
}
