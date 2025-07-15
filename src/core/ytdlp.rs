use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use tokio::process::Command as TokioCommand;

/// Video metadata extracted from yt-dlp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub title: String,
    pub duration: Option<f64>,
    pub uploader: Option<String>,
    pub upload_date: Option<String>,
    pub view_count: Option<u64>,
    pub thumbnail: Option<String>,
}

/// YT-DLP wrapper for managing video downloads
pub struct YtDlp {
    executable_path: Option<PathBuf>,
}

impl YtDlp {
    /// Create a new YT-DLP instance
    pub fn new() -> Self {
        Self {
            executable_path: None,
        }
    }

    /// Initialize yt-dlp (check if installed, install if needed)
    pub async fn initialize(&mut self) -> Result<()> {
        // First, try to find yt-dlp in PATH
        if let Ok(path) = which::which("yt-dlp") {
            self.executable_path = Some(path);
            return Ok(());
        }

        // If not found, try to install it
        self.install_ytdlp().await?;
        
        // Try to find it again after installation
        if let Ok(path) = which::which("yt-dlp") {
            self.executable_path = Some(path);
            Ok(())
        } else {
            Err(anyhow!("Failed to install or find yt-dlp"))
        }
    }

    /// Install yt-dlp using pip
    async fn install_ytdlp(&self) -> Result<()> {
        log::info!("Installing yt-dlp...");
        
        let output = TokioCommand::new("pip")
            .args(&["install", "yt-dlp"])
            .output()
            .await?;

        if output.status.success() {
            log::info!("yt-dlp installed successfully");
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to install yt-dlp: {}", error))
        }
    }

    /// Check if yt-dlp is available
    pub fn is_available(&self) -> bool {
        self.executable_path.is_some()
    }

    /// Get video metadata without downloading
    pub async fn get_metadata(&self, url: &str) -> Result<VideoMetadata> {
        let executable = self.executable_path
            .as_ref()
            .ok_or_else(|| anyhow!("yt-dlp not available"))?;

        let output = TokioCommand::new(executable)
            .args(&[
                "--dump-json",
                "--no-download",
                url
            ])
            .output()
            .await?;

        if output.status.success() {
            let json_str = String::from_utf8(output.stdout)?;
            let json_value: serde_json::Value = serde_json::from_str(&json_str)?;
            
            Ok(VideoMetadata {
                title: json_value["title"]
                    .as_str()
                    .unwrap_or("Unknown Title")
                    .to_string(),
                duration: json_value["duration"].as_f64(),
                uploader: json_value["uploader"]
                    .as_str()
                    .map(|s| s.to_string()),
                upload_date: json_value["upload_date"]
                    .as_str()
                    .map(|s| s.to_string()),
                view_count: json_value["view_count"].as_u64(),
                thumbnail: json_value["thumbnail"]
                    .as_str()
                    .map(|s| s.to_string()),
            })
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to get metadata: {}", error))
        }
    }

    /// Download a video
    pub async fn download_video(
        &self,
        url: &str,
        output_path: &PathBuf,
        resolution: &str,
    ) -> Result<String> {
        let executable = self.executable_path
            .as_ref()
            .ok_or_else(|| anyhow!("yt-dlp not available"))?;

        // Ensure output directory exists
        crate::utils::file_utils::ensure_dir_exists(output_path)?;

        // Build format selector for the desired resolution
        let format_selector = format!("best[height<={}]", 
            resolution.split('x').nth(1).unwrap_or("1080"));

        let output = TokioCommand::new(executable)
            .args(&[
                "--format", &format_selector,
                "--output", &format!("{}%(title)s.%(ext)s", output_path.to_string_lossy()),
                "--merge-output-format", "mp4",
                url
            ])
            .output()
            .await?;

        if output.status.success() {
            // Parse the output to get the actual filename
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Look for the download completion message
            for line in stdout.lines() {
                if line.contains("has already been downloaded") || 
                   line.contains("Destination:") {
                    // Extract filename from the line
                    if let Some(filename) = self.extract_filename_from_output(line) {
                        return Ok(filename);
                    }
                }
            }
            
            // Fallback: return a generic success message
            Ok("Download completed".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Download failed: {}", error))
        }
    }

    /// Extract filename from yt-dlp output
    fn extract_filename_from_output(&self, line: &str) -> Option<String> {
        // This is a simple implementation - in practice, you might want more robust parsing
        if let Some(start) = line.find("Destination: ") {
            let filename_part = &line[start + 13..];
            Some(filename_part.trim().to_string())
        } else {
            None
        }
    }

    /// Get available formats for a video
    pub async fn get_formats(&self, url: &str) -> Result<Vec<String>> {
        let executable = self.executable_path
            .as_ref()
            .ok_or_else(|| anyhow!("yt-dlp not available"))?;

        let output = TokioCommand::new(executable)
            .args(&["--list-formats", url])
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let formats: Vec<String> = stdout
                .lines()
                .filter(|line| line.contains("mp4") || line.contains("webm"))
                .map(|line| line.to_string())
                .collect();
            
            Ok(formats)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to get formats: {}", error))
        }
    }
}

impl Default for YtDlp {
    fn default() -> Self {
        Self::new()
    }
}
