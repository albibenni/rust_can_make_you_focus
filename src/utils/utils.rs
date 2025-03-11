use crate::FILE_PATH;

const YOUTUBE: &str = "youtube";
const X: &str = "x";
const NETFLIX: &str = "netflix";
const YOUTUBE_WEBSITE: &str = "www.youtube.com";
const NETFLIX_WEBSITE: &str = "www.netflix.com";
const X_WEBSITE: &str = "www.x.com";
const ALL: &str = "all";
const CODING: &str = "coding";
const STUDYING: &str = "studying";

/// Matches command line arguments and converts them to corresponding website URLs.
///
/// This function processes command line arguments and maps recognized platform identifiers
/// to their respective website URLs. Arguments that don't match any known platform are ignored.
///
/// # Returns
///
/// A vector of strings containing the website URLs for the matched platforms.
pub fn match_args(arguments: &Vec<String>) -> Vec<&str> {
    let supported_preset: Vec<&str> = vec![ALL, STUDYING, CODING];
    let mut vec_arg_websites: Vec<&str> = Vec::new();
    for arg in arguments {
        if supported_preset.contains(&arg.to_lowercase().as_str()) {
            vec_arg_websites = add_website_based_on_preset(&arg);
        }
        match arg.to_lowercase().as_str() {
            YOUTUBE => vec_arg_websites.push(YOUTUBE_WEBSITE),
            NETFLIX => vec_arg_websites.push(NETFLIX_WEBSITE),
            X => vec_arg_websites.push(X_WEBSITE),
            _ => continue,
        }
    }
    return vec_arg_websites;
}

/// Resets the contents of the main file to match the contents of the specified file.
///
/// # Arguments
///
/// * `file_path` - The path to the file whose contents will be used to reset the main file.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - Ok if the file was reset successfully, Err otherwise.
///
/// # Panics
///
/// Panics if the specified file does not exist or cannot be read.
pub fn reset_file(file_path: &str) -> Result<(), std::io::Error> {
    println!("{}", file_path);
    let file_reset = std::fs::read_to_string(file_path).expect("No such file or directory!");
    let res = std::fs::write(FILE_PATH, &file_reset);
    return res;
}

/// Generates a list of website URLs based on a specified preset category.
///
/// This function takes a preset category name and returns a vector of website URLs
/// that correspond to that category. Each preset represents a different use case
/// or focus area with relevant websites.
///
/// # Arguments
///
/// * `preset` - A string representing the preset category to use
///
/// # Returns
///
/// A vector of string slices containing website URLs for the specified preset
///
/// # Panics
///
/// Panics if the provided preset is not recognized
fn add_website_based_on_preset(preset: &String) -> Vec<&str> {
    let mut webs: Vec<&str> = Vec::new();
    match preset.to_lowercase().as_str() {
        ALL => {
            webs.push(YOUTUBE_WEBSITE);
            webs.push(X_WEBSITE);
            webs.push(NETFLIX_WEBSITE);
        }
        CODING => {
            webs.push(YOUTUBE_WEBSITE);
            webs.push(X_WEBSITE);
        }
        STUDYING => {
            webs.push(YOUTUBE_WEBSITE);
            webs.push(X_WEBSITE);
            webs.push(NETFLIX_WEBSITE);
        }
        _ => {
            panic!("preset not defined, consider adding a new preset");
        }
    }
    return webs;
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
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
    }
    #[test]
    fn test_match_args_with_preset() {
        let args = vec![ALL.to_string()];
        let result = match_args(&args);
        println!("AAAAAA {:?}", result);
        assert_eq!(result.len(), 3, "Should return three website URLs");
        assert!(
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
    }

    #[test]
    fn test_match_args_with_preset_study() {
        let args = vec![STUDYING.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 3, "Should return three website URLs");
        assert!(
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
    }

    #[test]
    fn test_match_args_with_preset_coding() {
        let args = vec![CODING.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 2, "Should return three website URLs");
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
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

    #[test]
    fn test_add_website_based_on_preset_all() {
        let preset = ALL.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(websites.len(), 3, "ALL preset should return 3 websites");
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "ALL preset should contain YouTube"
        );
        assert!(websites.contains(&X_WEBSITE), "ALL preset should contain X");
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "ALL preset should contain Netflix"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_coding() {
        let preset = CODING.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(websites.len(), 2, "CODING preset should return 2 websites");
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "CODING preset should contain YouTube"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "CODING preset should contain X"
        );
        assert!(
            !websites.contains(&NETFLIX_WEBSITE),
            "CODING preset should not contain Netflix"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_studying() {
        let preset = STUDYING.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(websites.len(), 3, "STUDING preset should return 3 websites");
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "STUDING preset should contain YouTube"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "STUDING preset should contain X"
        );
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "STUDING preset should contain Netflix"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_case_insensitive() {
        let preset = "CoDiNg".to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            2,
            "Case insensitive CODING preset should return 2 websites"
        );
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "Case insensitive CODING preset should contain YouTube"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "Case insensitive CODING preset should contain X"
        );
    }

    #[test]
    #[should_panic(expected = "preset not defined")]
    fn test_add_website_based_on_preset_invalid() {
        let preset = "INVALID_PRESET".to_string();
        add_website_based_on_preset(&preset); // Should panic
    }
}
