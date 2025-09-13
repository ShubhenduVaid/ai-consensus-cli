use serde::Deserialize;
use std::collections::HashMap;
use crate::{CliError, Result, Validator};
use log::info;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub tools: HashMap<String, ToolConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ToolConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub description: String,
}

impl Config {
    /// Loads configuration from a TOML file with security validation.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use ai_consensus_cli::Config;
    /// 
    /// let config = Config::load("config.toml").unwrap();
    /// assert!(!config.tools.is_empty());
    /// ```
    pub fn load(config_path: &str) -> Result<Self> {
        let home_dir = std::env::var("HOME").unwrap_or_default();
        let mut paths_to_try = vec![
            format!("{}/.config/ai-consensus-cli/config.toml", home_dir),
            config_path.to_string(),
        ];
        
        // Add binary location if different from config_path
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                if let Some(bin_path) = parent.join("config.toml").to_str() {
                    paths_to_try.push(bin_path.to_string());
                }
            }
        }
        
        for path in &paths_to_try {
            // Validate config path for security
            if let Ok(validated_path) = Validator::validate_config_path(&path) {
                if let Ok(config_content) = std::fs::read_to_string(&validated_path) {
                    let config: Config = toml::from_str(&config_content)
                        .map_err(|e| CliError::ConfigError { 
                            message: format!("Invalid config format: {}", e)
                        })?;
                    
                    config.validate()?;
                    info!("Loaded configuration from {} with {} tools", path, config.tools.len());
                    return Ok(config);
                }
            }
        }
        
        Err(CliError::ConfigError { 
            message: format!("Could not read config file: {} (tried: ~/.config/ai-consensus-cli/config.toml, current dir, binary location)", config_path)
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.tools.is_empty() {
            return Err(CliError::ConfigError { 
                message: "Configuration must contain at least one tool".to_string()
            });
        }
        
        for (key, tool) in &self.tools {
            if tool.name.trim().is_empty() {
                return Err(CliError::ConfigError { 
                    message: format!("Tool '{}' has empty name", key)
                });
            }
            if tool.command.trim().is_empty() {
                return Err(CliError::ConfigError { 
                    message: format!("Tool '{}' has empty command", key)
                });
            }
            if tool.description.trim().is_empty() {
                return Err(CliError::ConfigError { 
                    message: format!("Tool '{}' has empty description", key)
                });
            }
            
            // Validate command is allowed
            Validator::validate_command(&tool.command)?;
        }
        
        Ok(())
    }

    pub fn generate_help_text(&self) -> String {
        let mut help = String::from("Orchestrate multiple AI CLIs with consensus functionality.\n\nAvailable AI Tools:\n");
        
        for (key, tool) in &self.tools {
            help.push_str(&format!("• {:<12} - {}: {}\n", key, tool.name, tool.description));
        }
        
        help.push_str("\nExamples:\n");
        help.push_str("  ai-co -s q,gemini -c claude -p \"Explain microservices architecture\"\n");
        help.push_str("  ai-co -s q,ollama -c claude -p \"Latest AI developments\"\n");
        help.push_str("  ai-co -s q,gemini,claude,ollama -c q -p \"Your question here\"");
        
        help
    }
}
