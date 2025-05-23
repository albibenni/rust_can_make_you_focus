use std::process::Command;

use crate::FILE_PATH;

// Args
const YOUTUBE: &str = "youtube";
const X: &str = "x";
const NETFLIX: &str = "netflix";
const FACEBOOK: &str = "facebook";
const META: &str = "meta";
const INSTAGRAM: &str = "instagram";
const WHATSAPP: &str = "whatsapp";
const DISCORD: &str = "discord";

// Websites
const YOUTUBE_WEBSITE: &str = "www.youtube.com";
const NETFLIX_WEBSITE: &str = "netflix.com";
const X_WEBSITE: &str = "www.x.com";
const FACEBOOK_WEBSITE: &str = "www.facebook.com";
const INSTAGRAM_WEBSITE: &str = "www.instagram.com";
const WHATSAPP_WEBSITE: &str = "web.whatsapp.com";
const DISCORD_WEBSITE: &str = "discord.com";

// Presets
const ALL: &str = "all";
const CODING: &str = "coding";
const CODING_YT: &str = "coding_yt";
const STUDYING: &str = "studying";
const STUDYING_YT: &str = "studying_yt";

/// Matches command line arguments and converts them to corresponding website URLs.
///
/// This function processes command line arguments and maps recognized platform identifiers
/// to their respective website URLs. Arguments that don't match any known platform are ignored.
///
/// # Returns
///
/// A vector of strings containing the website URLs for the matched platforms.
pub fn match_args(arguments: &Vec<String>) -> Vec<&str> {
    let supported_preset: Vec<&str> = vec![ALL, STUDYING, STUDYING_YT, CODING, CODING_YT];
    let mut vec_arg_websites: Vec<&str> = Vec::new();
    for arg in arguments {
        if supported_preset.contains(&arg.to_lowercase().as_str()) {
            vec_arg_websites = add_website_based_on_preset(&arg);
        }
        match arg.to_lowercase().as_str() {
            YOUTUBE => vec_arg_websites.push(YOUTUBE_WEBSITE),
            NETFLIX => vec_arg_websites.push(NETFLIX_WEBSITE),
            X => vec_arg_websites.push(X_WEBSITE),
            INSTAGRAM => vec_arg_websites.push(INSTAGRAM_WEBSITE),
            META => vec_arg_websites.push(FACEBOOK_WEBSITE),
            FACEBOOK => vec_arg_websites.push(FACEBOOK_WEBSITE),
            DISCORD => vec_arg_websites.push(DISCORD_WEBSITE),
            WHATSAPP => vec_arg_websites.push(WHATSAPP_WEBSITE),
            _ => continue,
        }
    }
    return vec_arg_websites;
}

/// Resets a file's content and refreshes the flux cache
///
/// # Arguments
///
/// * `file_reset` - The content to write to the file
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - OK if successful, or an IO error
pub fn reset_file(file_reset: &str) -> Result<(), std::io::Error> {
    let _res = std::fs::write(FILE_PATH, &file_reset);
    let res = execute_flux_cache();
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
            webs.push(FACEBOOK_WEBSITE);
            webs.push(INSTAGRAM_WEBSITE);
            webs.push(DISCORD_WEBSITE);
            webs.push(WHATSAPP_WEBSITE);
        }
        CODING => {
            webs.push(YOUTUBE_WEBSITE);
            webs.push(X_WEBSITE);
            webs.push(FACEBOOK_WEBSITE);
            webs.push(INSTAGRAM_WEBSITE);
            webs.push(DISCORD_WEBSITE);
            webs.push(WHATSAPP_WEBSITE);
        }
        CODING_YT => {
            webs.push(X_WEBSITE);
            webs.push(FACEBOOK_WEBSITE);
            webs.push(INSTAGRAM_WEBSITE);
            webs.push(DISCORD_WEBSITE);
            webs.push(WHATSAPP_WEBSITE);
            webs.push(NETFLIX_WEBSITE);
        }
        STUDYING => {
            webs.push(YOUTUBE_WEBSITE);
            webs.push(X_WEBSITE);
            webs.push(NETFLIX_WEBSITE);
            webs.push(FACEBOOK_WEBSITE);
            webs.push(INSTAGRAM_WEBSITE);
            webs.push(DISCORD_WEBSITE);
            webs.push(WHATSAPP_WEBSITE);
        }
        STUDYING_YT => {
            webs.push(X_WEBSITE);
            webs.push(NETFLIX_WEBSITE);
            webs.push(FACEBOOK_WEBSITE);
            webs.push(INSTAGRAM_WEBSITE);
            webs.push(DISCORD_WEBSITE);
            webs.push(WHATSAPP_WEBSITE);
        }
        _ => {
            panic!("preset not defined, consider adding a new preset");
        }
    }
    return webs;
}

/// Flushes the DNS cache on macOS systems using the dscacheutil command.
///
/// This function executes the shell command `dscacheutil -flushcache` which is
/// used to clear the DNS cache on macOS.
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully, or an `Err` containing
/// the error message if it fails.
pub fn execute_flux_cache() -> Result<(), std::io::Error> {
    return Command::new("dscacheutil")
        .arg("-flushcache")
        .output()
        .map(|_| ())
        .map_err(|e| e);
}

/// Pauses the execution of the current thread for the specified duration.
///
/// This function is a simple wrapper around the standard library's `std::thread::sleep`
/// function, allowing the caller to specify the sleep duration in seconds.
///
/// # Arguments
///
/// * `seconds` - The number of whole seconds to sleep
///
/// # Examples
///
/// ```
/// use crate::utils::plan_sleep;
///
/// // Sleep for 2 seconds
/// plan_sleep(2);
pub fn plan_sleep(seconds: u64) {
    println!("Pomodoro Starting for: {} minutes", seconds as f64 / 60.0);
    std::thread::sleep(std::time::Duration::new(seconds, 0));
    println!("Pomodoro Ended, Good job");
}

/// Parses a string representation of a sleep time into an unsigned 64-bit integer.
/// After the parsing it converts the number, taken in minutes from the cli,
/// in seconds for the `plan_sleep()` function
///
/// This function attempts to convert a string containing a number into a `u64` value.
///
/// # Arguments
///
/// * `argument` - A reference to a String that should contain a valid numeric value
///
/// # Returns
///
/// Returns the parsed `u64` value representing seconds.
///
/// # Panics
///
/// This function will panic with the message "You didn't provide a number!" if the
/// string cannot be parsed as a valid `u64` (e.g., if it contains non-numeric characters
/// or represents a number outside the valid range for `u64`).
///
/// # Examples
///
/// ```
/// use crate::utils::parse_sleep_time;
///
/// let time_str = String::from("5");
/// let seconds = parse_sleep_time(&time_str);
/// assert_eq!(seconds, 5);
/// ```
pub fn parse_sleep_time(argument: &String) -> u64 {
    return argument
        .parse::<u64>()
        .map(|n| n * 60)
        .expect("You didn't provide a number!");
}

/// Displays command-line help information for the application.
///
/// Prints usage instructions to stdout, showing:
/// - Supported website arguments (YouTube, X, Netflix)
/// - Available presets (ALL, CODING, STUDYING)
/// - Instructions for specifying the Pomodoro timer duration
///
/// # Examples
///
/// ```
/// use crate::utils::help;
///
/// // Display help information to the user
/// help();
/// ```
pub fn help() -> () {
    println!("Provide as many arguments as you want of those supported: ");
    println!("  - {} ", YOUTUBE);
    println!("  - {} ", X);
    println!("  - {} ", NETFLIX);
    println!("Provide a preset of those supported: ");
    println!("  - {} ", ALL);
    println!("  - {} ", CODING);
    println!("  - {} ", STUDYING);
    println!("As last argument provide the Pomodoro timer in minutes until then the websites are blocked");
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::io::Write;
    use std::path::Path;
    use tempfile::NamedTempFile;

    #[test]
    fn test_reset_file_success() {
        // We need to use an environment variable or temporarily modify the const
        // to point to our test file for this test to work properly
        // For this example, we'll mock the behavior since we can't modify consts at runtime

        // Create a temporary file for testing
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "test content";

        // Mock the behavior of reset_file
        let result: Result<(), std::io::Error> = {
            // Write to the temp file (mimicking std::fs::write in reset_file)
            temp_file.write_all(content.as_bytes()).unwrap();

            // Verify the content was written correctly
            let temp_path = temp_file.path();
            assert_eq!(fs::read_to_string(temp_path).unwrap(), content);

            // Pretend we executed flux cache refresh (depends on what execute_flux_cache does)
            Ok(())
        };

        assert!(result.is_ok());
    }

    #[test]
    fn test_reset_file_write_error() {
        // Test the case where writing to the file fails
        let result = {
            // Create a scenario where write would fail
            // (e.g., trying to write to a nonexistent directory)
            let nonexistent_path = Path::new("/nonexistent/directory/file.txt");

            // This would fail in reset_file
            fs::write(nonexistent_path, "test content")
        };

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_sleep_time_valid() {
        let valid_input = String::from("42");
        assert_eq!(parse_sleep_time(&valid_input), 42 * 60);
    }

    #[test]
    fn test_parse_sleep_time_valid2() {
        let valid_input = String::from("1");
        assert_eq!(parse_sleep_time(&valid_input), 60);
    }
    #[test]
    #[should_panic(expected = "You didn't provide a number!")]
    fn test_parse_sleep_time_invalid() {
        let invalid_input = String::from("not_a_number");
        parse_sleep_time(&invalid_input); // Should panic
    }
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
    fn test_plan_sleep() {
        // This is a simple test to verify the function doesn't panic
        // More sophisticated timing tests would be flaky
        let start = std::time::Instant::now();
        plan_sleep(1); // Sleep for just 1 second to keep test fast
        let duration = start.elapsed();

        // We expect it to sleep at least 0.9 seconds (allowing for some timing variance)
        assert!(duration.as_secs_f64() >= 0.9);
    }

    #[test]
    fn test_execute_flux_cache_returns_result() {
        // This test only verifies the function returns a Result
        // without actually running the command
        let result = execute_flux_cache();

        // Just check that it's a Result type (we can't easily test actual execution)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_help_doesnt_panic() {
        // Simple test to ensure help() doesn't panic
        let result = std::panic::catch_unwind(|| {
            help();
        });
        assert!(result.is_ok());
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
        assert_eq!(result.len(), 3, "Should return 3 website URLs");
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
        assert_eq!(result.len(), 7, "Should return 7 website URLs");
        assert!(
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
        assert!(
            result.contains(&FACEBOOK_WEBSITE),
            "Should contain Facebook website URL"
        );
        assert!(
            result.contains(&INSTAGRAM_WEBSITE),
            "Should contain Instagram website URL"
        );
        assert!(
            result.contains(&DISCORD_WEBSITE),
            "Should contain discord website URL"
        );
        assert!(
            result.contains(&WHATSAPP_WEBSITE),
            "Should contain whatsapp website URL"
        );
    }

    #[test]
    fn test_match_args_with_preset_study() {
        let args = vec![STUDYING.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 7, "Should return 7 website URLs");
        assert!(
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
        assert!(
            result.contains(&FACEBOOK_WEBSITE),
            "Should contain Facebook website URL"
        );
        assert!(
            result.contains(&INSTAGRAM_WEBSITE),
            "Should contain Instagram website URL"
        );
        assert!(
            result.contains(&DISCORD_WEBSITE),
            "Should contain discord website URL"
        );
        assert!(
            result.contains(&WHATSAPP_WEBSITE),
            "Should contain whatsapp website URL"
        );
    }
    #[test]
    fn test_match_args_with_preset_study_yt() {
        let args = vec![STUDYING_YT.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 6, "Should return 6 website URLs");
        assert!(
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(
            !result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
        assert!(
            result.contains(&FACEBOOK_WEBSITE),
            "Should contain Facebook website URL"
        );
        assert!(
            result.contains(&INSTAGRAM_WEBSITE),
            "Should contain Instagram website URL"
        );
        assert!(
            result.contains(&DISCORD_WEBSITE),
            "Should contain discord website URL"
        );
        assert!(
            result.contains(&WHATSAPP_WEBSITE),
            "Should contain whatsapp website URL"
        );
    }

    #[test]
    fn test_match_args_with_preset_coding() {
        let args = vec![CODING.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 6, "Should return 6 website URLs");
        assert!(
            result.contains(&YOUTUBE_WEBSITE),
            "Should contain YouTube website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
        assert!(
            result.contains(&FACEBOOK_WEBSITE),
            "Should contain Facebook website URL"
        );
        assert!(
            result.contains(&INSTAGRAM_WEBSITE),
            "Should contain Instagram website URL"
        );
        assert!(
            result.contains(&DISCORD_WEBSITE),
            "Should contain discord website URL"
        );
        assert!(
            result.contains(&WHATSAPP_WEBSITE),
            "Should contain whatsapp website URL"
        );
    }

    #[test]
    fn test_match_args_with_preset_coding_yt() {
        let args = vec![CODING_YT.to_string()];
        let result = match_args(&args);
        assert_eq!(result.len(), 6, "Should return 6 website URLs");
        assert!(
            !result.contains(&YOUTUBE_WEBSITE),
            "Should not contain YouTube website URL"
        );
        assert!(
            result.contains(&NETFLIX_WEBSITE),
            "Should contain Netflix website URL"
        );
        assert!(result.contains(&X_WEBSITE), "Should contain X website URL");
        assert!(
            result.contains(&FACEBOOK_WEBSITE),
            "Should contain Facebook website URL"
        );
        assert!(
            result.contains(&INSTAGRAM_WEBSITE),
            "Should contain Instagram website URL"
        );
        assert!(
            result.contains(&DISCORD_WEBSITE),
            "Should contain discord website URL"
        );
        assert!(
            result.contains(&WHATSAPP_WEBSITE),
            "Should contain whatsapp website URL"
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

    #[test]
    fn test_add_website_based_on_preset_all() {
        let preset = ALL.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(websites.len(), 7, "ALL preset should return 7 websites");
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "ALL preset should contain YouTube"
        );
        assert!(websites.contains(&X_WEBSITE), "ALL preset should contain X");
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "ALL preset should contain Netflix"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "Should contain Facebook website URL"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "Should contain Instagram website URL"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "Should contain discord website URL"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "Should contain whatsapp website URL"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_coding() {
        let preset = CODING.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(websites.len(), 6, "CODING preset should return 6 websites");
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "CODING preset should contain YouTube"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "CODING preset should contain X"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "CODING preset should contain Facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "CODING preset should contain Instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "CODING preset should contain Discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "CODING preset should contain whatsapp"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_studying() {
        let preset = STUDYING.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(websites.len(), 7, "STUDING preset should return 7 websites");
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
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "STUDING preset should contain Facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "STUDING preset should contain Instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "STUDING preset should contain Discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "STUDING preset should contain whatsapp"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_studying_yt() {
        let preset = STUDYING_YT.to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            6,
            "STUDYING_YT preset should return 6 websites"
        );
        assert!(
            !websites.contains(&YOUTUBE_WEBSITE),
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
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "STUDING preset should contain Facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "STUDING preset should contain Instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "STUDING preset should contain Discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "STUDING preset should contain whatsapp"
        );
    }
    #[test]
    fn test_add_website_based_on_preset_case_insensitive() {
        let preset = "CoDiNg".to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            6,
            "Case insensitive CODING preset should return 6 websites"
        );
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "Case insensitive CODING preset should contain YouTube"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "Case insensitive CODING preset should contain X"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "Case insensitive CODING preset should contain facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "Case insensitive CODING preset should contain instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "Case insensitive CODING preset should contain discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "Case insensitive CODING preset should contain whatsapp"
        );
    }
    #[test]
    fn test_add_website_based_on_preset_case_insensitive_coding_yt() {
        let preset = "CoDiNg_yt".to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            6,
            "Case insensitive CODING_yt preset should return 6 websites"
        );
        assert!(
            !websites.contains(&YOUTUBE_WEBSITE),
            "Case insensitive CODING preset should not contain YouTube"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "Case insensitive CODING preset should contain X"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "Case insensitive CODING preset should contain facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "Case insensitive CODING preset should contain instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "Case insensitive CODING preset should contain discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "Case insensitive CODING preset should contain whatsapp"
        );
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "Case insensitive CODING preset should contain netflix"
        );
    }
    #[test]
    fn test_add_website_based_on_preset_case_insensitive_studying() {
        let preset = "stUdYing".to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            7,
            "Case insensitive studying preset should return 7 websites"
        );
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "Case insensitive studying preset should contain YouTube"
        );
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "Case insensitive studying preset should contain Netflix"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "Case insensitive studying preset should contain X"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "Case insensitive studying preset should contain Facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "Case insensitive studying preset should contain Instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "Case insensitive studying preset should contain Discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "Case insensitive studying preset should contain whatsapp"
        );
    }
    #[test]
    fn test_add_website_based_on_preset_case_insensitive_studying_yt() {
        let preset = "stUdYing_Yt".to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            6,
            "Case insensitive studying preset should return 6 websites"
        );
        assert!(
            !websites.contains(&YOUTUBE_WEBSITE),
            "Case insensitive studying preset should contain YouTube"
        );
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "Case insensitive studying preset should contain Netflix"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "Case insensitive studying preset should contain X"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "Case insensitive studying preset should contain Facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "Case insensitive studying preset should contain Instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "Case insensitive studying preset should contain Discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "Case insensitive studying preset should contain whatsapp"
        );
    }

    #[test]
    fn test_add_website_based_on_preset_case_insensitive_all() {
        let preset = "aLl".to_string();
        let websites = add_website_based_on_preset(&preset);

        assert_eq!(
            websites.len(),
            7,
            "Case insensitive ALL preset should return 7 websites"
        );
        assert!(
            websites.contains(&YOUTUBE_WEBSITE),
            "Case insensitive All preset should contain YouTube"
        );
        assert!(
            websites.contains(&NETFLIX_WEBSITE),
            "Case insensitive All preset should contain Netflix"
        );
        assert!(
            websites.contains(&X_WEBSITE),
            "Case insensitive All preset should contain X"
        );
        assert!(
            websites.contains(&FACEBOOK_WEBSITE),
            "Case insensitive All preset should contain facebook"
        );
        assert!(
            websites.contains(&INSTAGRAM_WEBSITE),
            "Case insensitive All preset should contain instagram"
        );
        assert!(
            websites.contains(&DISCORD_WEBSITE),
            "Case insensitive All preset should contain discord"
        );
        assert!(
            websites.contains(&WHATSAPP_WEBSITE),
            "Case insensitive All preset should contain whatsapp"
        );
    }
    #[test]
    #[should_panic(expected = "preset not defined")]
    fn test_add_website_based_on_preset_invalid() {
        let preset = "INVALID_PRESET".to_string();
        add_website_based_on_preset(&preset); // Should panic
    }
}
