pub const TOOL_TIMEOUT_SECS: u64 = 60;
pub const MAX_PROMPT_LENGTH: usize = 50000;
pub const MEMORY_LIMIT_MB: u64 = 512;
pub const CPU_LIMIT_SECS: u64 = 60;

pub const ALLOWED_COMMANDS: &[&str] = &[
    "q", "gemini", "claude", "openai", "ollama", 
    "mistral"
];

pub const AUTH_ERROR_PATTERNS: &[&str] = &[
    "invalid api key",
    "api_key client option must be set", 
    "please run /login",
    "authentication",
    "api key"
];
