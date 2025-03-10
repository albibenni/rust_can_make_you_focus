const YOUTUBE: &str = "youtube";
const X: &str = "x";
const NETFLIX: &str = "netflix";
const YOUTUBE_WEBSITE: &str = "www.youtube.com";
const NETFLIX_WEBSITE: &str = "www.netflix.com";
const X_WEBSITE: &str = "www.x.com";

/// Matches command line arguments and converts them to corresponding website URLs.
///
/// This function processes command line arguments and maps recognized platform identifiers
/// to their respective website URLs. Arguments that don't match any known platform are ignored.
///
/// # Returns
///
/// A vector of strings containing the website URLs for the matched platforms.
pub fn match_args(arguments: &Vec<String>) -> Vec<String> {
    let mut vec_arg_websites: Vec<String> = Vec::new();
    for arg in arguments {
        match arg.to_lowercase().as_str() {
            YOUTUBE => vec_arg_websites.push(YOUTUBE_WEBSITE.to_string()),
            NETFLIX => vec_arg_websites.push(NETFLIX_WEBSITE.to_string()),
            X => vec_arg_websites.push(X_WEBSITE.to_string()),
            _ => continue,
        }
    }
    return vec_arg_websites;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_args_empty() {
        let args: Vec<String> = Vec::new();
        let result = match_args(&args);
        assert_eq!(
            result.len(),
            0,
            "Should return empty vector when no arguments are provided"
        );
    }

    #[test]
    fn test_match_args_with_youtube() {
        let args = vec![YOUTUBE.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 1, "Should return one website URL");
        assert_eq!(
            result[0], YOUTUBE_WEBSITE,
            "Should return YouTube website URL"
        );
    }

    #[test]
    fn test_match_args_with_x() {
        let args = vec![X.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 1, "Should return one website URL");
        assert_eq!(result[0], X_WEBSITE, "Should return X website URL");
    }

    #[test]
    fn test_match_args_with_netflix() {
        let args = vec![NETFLIX.to_uppercase().to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 1, "Should return one website URL");
        assert_eq!(result[0], NETFLIX_WEBSITE, "Should return X website URL");
    }
    #[test]
    fn test_match_args_with_multiple_platforms() {
        let args = vec![
            NETFLIX.to_uppercase().to_string(),
            YOUTUBE.to_string(),
            X.to_string(),
        ];
        let result = match_args(&args);
        assert_eq!(result.len(), 3, "Should return two website URLs");
        assert!(
            result.contains(&NETFLIX_WEBSITE.to_string()),
            "Should contain Netflix website URL"
        );
        assert!(
            result.contains(&YOUTUBE_WEBSITE.to_string()),
            "Should contain YouTube website URL"
        );
        assert!(
            result.contains(&X_WEBSITE.to_string()),
            "Should contain X website URL"
        );
    }

    #[test]
    fn test_match_args_with_unknown_platform() {
        let args = vec!["unknown".to_string()];
        let result = match_args(&args);
        assert_eq!(
            result.len(),
            0,
            "Should return empty vector for unknown platforms"
        );
    }
}
