use std::process::{Command, Output};

fn get_project_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

fn run_cli(args: &[&str]) -> Output {
    Command::new("cargo")
        .args(args)
        .current_dir(get_project_dir())
        .env(
            "AI_CONSENSUS_CONFIG",
            format!("{}/config.toml", get_project_dir()),
        )
        .output()
        .expect("Failed to execute command")
}

#[test]
fn test_cli_help() {
    let output = run_cli(&["run", "--", "-s", "", "-c", "", "-p", ""]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available AI Tools:"));
    assert!(stdout.contains("Amazon Q: AWS AI assistant"));
    assert!(stdout.contains("Google Gemini: Strong general-purpose AI"));
    assert!(stdout.contains("OpenAI Codex"));
    assert!(stdout.contains("Examples:"));
}

#[test]
fn test_cli_missing_required_args() {
    let output = run_cli(&["run", "--", "-s", "q"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("missing"));
}

#[test]
fn test_cli_invalid_solver() {
    let output = run_cli(&["run", "--", "-s", "invalid-tool", "-c", "q", "-p", "test"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ToolNotFound") || stderr.contains("not found"));
}

#[test]
fn test_cli_invalid_consensus() {
    let output = run_cli(&["run", "--", "-s", "q", "-c", "invalid-tool", "-p", "test"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ToolNotFound") || stderr.contains("not found"));
}

#[test]
fn test_cli_multiple_solvers() {
    let output = run_cli(&["run", "--", "-s", "q,gemini", "-c", "q", "-p", "What is 2+2?"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should show progress indicators
    assert!(stdout.contains("🤖 Running") || stderr.contains("🤖 Running"));
    
    // Should show solver count
    assert!(stdout.contains("solver(s)") || stderr.contains("solver(s)"));
}

#[test]
fn test_cli_single_solver() {
    let output = run_cli(&["run", "--", "-s", "ollama", "-c", "ollama", "-p", "What is 1+1?"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should show running 1 solver
    assert!(stdout.contains("Running 1 solver") || stderr.contains("Running 1 solver"));
}

#[test]
fn test_cli_unavailable_tools() {
    let output = run_cli(&["run", "--", "-s", "mistral,codellama,codex", "-c", "q", "-p", "test"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should handle unavailable tools gracefully
    let combined_output = format!("{}{}", stdout, stderr);
    assert!(
        combined_output.contains("❌") || 
        combined_output.contains("Unavailable tools") ||
        combined_output.contains("All solvers failed") ||
        combined_output.contains("not available")
    );
}

#[test]
fn test_cli_status_indicators() {
    let output = run_cli(&["run", "--", "-s", "q,ollama", "-c", "q", "-p", "Hello"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show status indicators (✅ or ❌)
    assert!(stdout.contains("✅") || stdout.contains("❌"));
    
    // Should show timing information
    assert!(stdout.contains("(") && stdout.contains("s)"));
}

#[test]
fn test_cli_consensus_phase() {
    let output = run_cli(&["run", "--", "-s", "ollama", "-c", "ollama", "-p", "Say hello"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show consensus phase
    assert!(stdout.contains("🧠 Getting consensus"));
}

#[test]
fn test_cli_output_format() {
    let output = run_cli(&["run", "--", "-s", "ollama", "-c", "ollama", "-p", "What is the capital of France?"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    if output.status.success() {
        // Should show progress indicators
        assert!(stdout.contains("🤖 Running"));
        assert!(stdout.contains("🧠 Getting consensus"));
        
        // Should show timing
        assert!(stdout.contains("(") && stdout.contains("s)"));
    }
}

#[test]
fn test_cli_config_file() {
    let output = run_cli(&["run", "--", "--config", "config.toml", "-s", "q", "-c", "q", "-p", "test"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should either work or show config-related error
    let combined_output = format!("{}{}", stdout, stderr);
    assert!(
        combined_output.contains("🤖 Running") ||
        combined_output.contains("Could not read config") ||
        output.status.success()
    );
}
