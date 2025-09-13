use crate::{Config, CliError, Result, constants::*};
use lazy_static::lazy_static;
use std::path::{PathBuf, Component};

lazy_static! {
    static ref ANSI_REGEX: regex::Regex = regex::Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
}

pub struct Validator;

impl Validator {
    /// Sanitizes user input prompts by filtering dangerous characters.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ai_consensus_cli::Validator;
    /// 
    /// let result = Validator::sanitize_prompt("Hello world!").unwrap();
    /// assert_eq!(result, "Hello world!");
    /// 
    /// // Empty prompts are rejected
    /// assert!(Validator::sanitize_prompt("").is_err());
    /// ```
    pub fn sanitize_prompt(prompt: &str) -> Result<String> {
        if prompt.trim().is_empty() {
            return Err(CliError::InvalidPrompt { 
                reason: "Prompt cannot be empty".to_string() 
            });
        }
        
        if prompt.len() > MAX_PROMPT_LENGTH {
            return Err(CliError::InvalidPrompt { 
                reason: format!("Prompt too long (max {} characters)", MAX_PROMPT_LENGTH)
            });
        }
        
        let sanitized: String = prompt.chars()
            .filter(|c| c.is_alphanumeric() || " .,?!-_:;()[]{}\"'`\n\t".contains(*c))
            .collect();
        
        Ok(sanitized)
    }

    pub fn validate_config_path(path: &str) -> Result<PathBuf> {
        let path_buf = PathBuf::from(path);
        
        // Expand ~ to home directory
        let expanded_path = if path.starts_with("~/") {
            let home_dir = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(path.replacen("~", &home_dir, 1))
        } else {
            path_buf.clone()
        };
        
        // Allow absolute paths in home directory
        let home_dir = std::env::var("HOME").unwrap_or_default();
        if expanded_path.starts_with(&home_dir) {
            return Ok(expanded_path);
        }
        
        // Allow relative paths (current directory)
        if path_buf.is_relative() {
            // Check for dangerous path traversal
            for component in path_buf.components() {
                if let Component::ParentDir = component {
                    return Err(CliError::InvalidConfigPath { 
                        path: path.to_string() 
                    });
                }
            }
            return Ok(path_buf);
        }
        
        // Allow system paths
        if path.starts_with("/usr/local/") || path.starts_with("/opt/") {
            return Ok(path_buf);
        }
        
        // Default allow for other absolute paths (but be cautious)
        Ok(path_buf)
    }

    /// Validates that a command is in the allowlist for security.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ai_consensus_cli::Validator;
    /// 
    /// // Allowed commands pass
    /// assert!(Validator::validate_command("q").is_ok());
    /// assert!(Validator::validate_command("gemini").is_ok());
    /// 
    /// // Dangerous commands are blocked
    /// assert!(Validator::validate_command("rm").is_err());
    /// ```
    pub fn validate_command(command: &str) -> Result<()> {
        if !ALLOWED_COMMANDS.contains(&command) {
            return Err(CliError::CommandNotAllowed { 
                command: command.to_string() 
            });
        }
        Ok(())
    }

    pub fn validate_tools(solvers: &[String], consensus: &str, config: &Config) -> Result<()> {
        for solver in solvers {
            if !config.tools.contains_key(solver) {
                return Err(CliError::ToolNotFound { 
                    tool: solver.clone() 
                });
            }
            
            let tool_config = &config.tools[solver];
            Self::validate_command(&tool_config.command)?;
        }
        
        if !config.tools.contains_key(consensus) {
            return Err(CliError::ToolNotFound { 
                tool: consensus.to_string() 
            });
        }
        
        let consensus_config = &config.tools[consensus];
        Self::validate_command(&consensus_config.command)?;
        
        Ok(())
    }

    pub fn sanitize_args(args: &[String], prompt: &str) -> Result<Vec<String>> {
        let sanitized_prompt = Self::sanitize_prompt(prompt)?;
        let mut sanitized_args = Vec::new();
        
        for arg in args {
            if arg == "{prompt}" {
                sanitized_args.push(sanitized_prompt.clone());
            } else {
                if arg.contains(';') || arg.contains('|') || arg.contains('&') || arg.contains('`') {
                    return Err(CliError::InvalidPrompt { 
                        reason: "Invalid characters in arguments".to_string() 
                    });
                }
                sanitized_args.push(arg.clone());
            }
        }
        
        Ok(sanitized_args)
    }

    pub fn is_authentication_error(response: &str) -> bool {
        let response_lower = response.to_lowercase();
        AUTH_ERROR_PATTERNS.iter().any(|pattern| response_lower.contains(pattern))
    }

    /// Strips ANSI escape codes from text output.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ai_consensus_cli::Validator;
    /// 
    /// let input = "\u{001b}[31mRed text\u{001b}[0m Normal text";
    /// let result = Validator::strip_ansi_codes(input);
    /// assert_eq!(result, "Red text Normal text");
    /// ```
    pub fn strip_ansi_codes(text: &str) -> String {
        let cleaned = ANSI_REGEX.replace_all(text, "");
        cleaned.replace('\u{0007}', "").replace("  ", " ")
    }
}
