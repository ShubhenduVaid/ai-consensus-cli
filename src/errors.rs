use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Tool '{tool}' not found in configuration")]
    ToolNotFound { tool: String },
    
    #[error("Tool '{tool}' timed out after {timeout}s")]
    ToolTimeout { tool: String, timeout: u64 },
    
    #[error("Authentication failed for tool '{tool}': {reason}")]
    AuthenticationFailed { tool: String, reason: String },
    
    #[error("Command '{command}' not allowed")]
    CommandNotAllowed { command: String },
    
    #[error("Invalid config path: {path}")]
    InvalidConfigPath { path: String },
    
    #[error("Prompt validation failed: {reason}")]
    InvalidPrompt { reason: String },
    
    #[error("All solver tools failed")]
    AllSolversFailed,
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}

pub type Result<T> = std::result::Result<T, CliError>;
