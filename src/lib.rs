pub mod config;
pub mod ui;
pub mod tools;
pub mod consensus;
pub mod validation;
pub mod errors;
pub mod constants;

pub use config::{Config, ToolConfig};
pub use tools::ToolManager;
pub use consensus::ConsensusEngine;
pub use validation::Validator;
pub use errors::{CliError, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_config_validation() {
        // Valid config with allowed command
        let mut tools = HashMap::new();
        tools.insert("test".to_string(), ToolConfig {
            name: "Test Tool".to_string(),
            command: "q".to_string(), // Use allowed command
            args: vec!["{prompt}".to_string()],
            description: "A test tool".to_string(),
        });
        let config = Config { tools };
        assert!(config.validate().is_ok());
        
        // Empty tools
        let empty_config = Config { tools: HashMap::new() };
        assert!(empty_config.validate().is_err());
    }

    #[test]
    fn test_prompt_sanitization() {
        // Valid prompts
        assert!(Validator::sanitize_prompt("Hello world!").is_ok());
        assert!(Validator::sanitize_prompt("What is 2+2?").is_ok());
        assert_eq!(Validator::sanitize_prompt("Test prompt").unwrap(), "Test prompt");
        
        // Invalid prompts
        assert!(Validator::sanitize_prompt("").is_err());
        assert!(Validator::sanitize_prompt("   ").is_err());
        assert!(Validator::sanitize_prompt(&"x".repeat(50001)).is_err());
    }

    #[test]
    fn test_command_allowlist() {
        assert!(Validator::validate_command("q").is_ok());
        assert!(Validator::validate_command("gemini").is_ok());
        assert!(Validator::validate_command("malicious").is_err());
        assert!(Validator::validate_command("rm").is_err());
    }

    #[test]
    fn test_path_traversal_prevention() {
        assert!(Validator::validate_config_path("../../../etc/passwd").is_err());
        assert!(Validator::validate_config_path("config.toml").is_ok());
        assert!(Validator::validate_config_path("./config.toml").is_ok());
    }

    #[test]
    fn test_argument_sanitization() {
        let args = vec!["chat".to_string(), "{prompt}".to_string()];
        assert!(Validator::sanitize_args(&args, "Hello world").is_ok());
        
        let malicious_args = vec!["chat".to_string(), "; rm -rf /".to_string()];
        assert!(Validator::sanitize_args(&malicious_args, "test").is_err());
    }

    #[test]
    fn test_authentication_error_detection() {
        assert!(Validator::is_authentication_error("Invalid API key"));
        assert!(Validator::is_authentication_error("Please run /login"));
        assert!(Validator::is_authentication_error("api_key client option must be set"));
        assert!(!Validator::is_authentication_error("Hello world"));
    }

    #[test]
    fn test_ansi_code_stripping() {
        let input = "\u{001b}[31mRed text\u{001b}[0m\u{0007}Normal text";
        let expected = "Red textNormal text";
        assert_eq!(Validator::strip_ansi_codes(input), expected);
    }

    #[test]
    fn test_help_text_generation() {
        let mut tools = HashMap::new();
        tools.insert("test".to_string(), ToolConfig {
            name: "Test Tool".to_string(),
            command: "q".to_string(),
            args: vec!["{prompt}".to_string()],
            description: "A test tool".to_string(),
        });
        
        let config = Config { tools };
        let help = config.generate_help_text();
        
        assert!(help.contains("Available AI Tools:"));
        assert!(help.contains("Test Tool: A test tool"));
        assert!(help.contains("Examples:"));
    }
}
