use url::Url;

/// Validate if a URL is a valid YouTube URL
pub fn is_valid_youtube_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(url) => {
            let host = url.host_str().unwrap_or("");
            
            // Check for various YouTube domains
            matches!(host, 
                "www.youtube.com" | 
                "youtube.com" | 
                "youtu.be" | 
                "m.youtube.com" |
                "music.youtube.com"
            )
        }
        Err(_) => false,
    }
}

/// Extract video ID from YouTube URL
pub fn extract_video_id(url_str: &str) -> Option<String> {
    let url = Url::parse(url_str).ok()?;
    let host = url.host_str()?;
    
    match host {
        "youtu.be" => {
            // Format: https://youtu.be/VIDEO_ID
            url.path_segments()?.next().map(|s| s.to_string())
        }
        "www.youtube.com" | "youtube.com" | "m.youtube.com" => {
            // Format: https://www.youtube.com/watch?v=VIDEO_ID
            url.query_pairs()
                .find(|(key, _)| key == "v")
                .map(|(_, value)| value.to_string())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_youtube_urls() {
        assert!(is_valid_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(is_valid_youtube_url("https://youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(is_valid_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
        assert!(is_valid_youtube_url("https://m.youtube.com/watch?v=dQw4w9WgXcQ"));
    }

    #[test]
    fn test_invalid_urls() {
        assert!(!is_valid_youtube_url("https://www.google.com"));
        assert!(!is_valid_youtube_url("not a url"));
        assert!(!is_valid_youtube_url(""));
    }

    #[test]
    fn test_extract_video_id() {
        assert_eq!(
            extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }
}
