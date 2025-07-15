use std::path::PathBuf;

/// Get the default downloads directory for the current platform
pub fn get_downloads_dir() -> PathBuf {
    // Try to get the user's Downloads directory
    if let Some(downloads_dir) = dirs::download_dir() {
        downloads_dir.join("ytdl-mini")
    } else {
        // Fallback to home directory
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Downloads")
            .join("ytdl-mini")
    }
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &PathBuf) -> Result<(), anyhow::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Generate a safe filename from a video title
pub fn sanitize_filename(title: &str) -> String {
    // Remove or replace characters that are not safe for filenames
    let mut safe_name = title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>();
    
    // Trim whitespace and limit length
    safe_name = safe_name.trim().to_string();
    if safe_name.len() > 200 {
        safe_name.truncate(200);
    }
    
    // Ensure we have a non-empty filename
    if safe_name.is_empty() {
        safe_name = "video".to_string();
    }
    
    safe_name
}

/// Get file extension for the given format
pub fn get_file_extension(format: &str) -> &str {
    match format.to_lowercase().as_str() {
        "mp4" => "mp4",
        "webm" => "webm",
        "mkv" => "mkv",
        "avi" => "avi",
        "mov" => "mov",
        _ => "mp4", // Default to mp4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Normal Title"), "Normal Title");
        assert_eq!(sanitize_filename("Title/With\\Bad:Chars"), "Title_With_Bad_Chars");
        assert_eq!(sanitize_filename(""), "video");
        assert_eq!(sanitize_filename("   "), "video");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("mp4"), "mp4");
        assert_eq!(get_file_extension("MP4"), "mp4");
        assert_eq!(get_file_extension("unknown"), "mp4");
    }
}
