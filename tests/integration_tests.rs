use std::process::Command;

fn get_project_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "", "-c", "", "-p", ""])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available AI Tools:"));
    assert!(stdout.contains("Amazon Q: AWS AI assistant"));
    assert!(stdout.contains("Google Gemini: Strong general-purpose AI"));
    assert!(stdout.contains("Examples:"));
}

#[test]
fn test_cli_missing_required_args() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "q"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("missing"));
}

#[test]
fn test_cli_invalid_solver() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "invalid-tool", "-c", "q", "-p", "test"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ToolNotFound") || stderr.contains("not found"));
}

#[test]
fn test_cli_invalid_consensus() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "q", "-c", "invalid-tool", "-p", "test"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ToolNotFound") || stderr.contains("not found"));
}

#[test]
fn test_cli_multiple_solvers() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "q,gemini", "-c", "q", "-p", "What is 2+2?"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should show progress indicators
    assert!(stdout.contains("🤖 Running") || stderr.contains("🤖 Running"));
    
    // Should show solver count
    assert!(stdout.contains("solver(s)") || stderr.contains("solver(s)"));
}

#[test]
fn test_cli_single_solver() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "ollama", "-c", "ollama", "-p", "What is 1+1?"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should show running 1 solver
    assert!(stdout.contains("Running 1 solver") || stderr.contains("Running 1 solver"));
}

#[test]
fn test_cli_unavailable_tools() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "mistral,codellama", "-c", "q", "-p", "test"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

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
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "q,ollama", "-c", "q", "-p", "Hello"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show status indicators (✅ or ❌)
    assert!(stdout.contains("✅") || stdout.contains("❌"));
    
    // Should show timing information
    assert!(stdout.contains("(") && stdout.contains("s)"));
}

#[test]
fn test_cli_consensus_phase() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "ollama", "-c", "ollama", "-p", "Say hello"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show consensus phase
    assert!(stdout.contains("🧠 Getting consensus"));
}

#[test]
fn test_cli_output_format() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-s", "ollama", "-c", "ollama", "-p", "What is the capital of France?"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

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
    let output = Command::new("cargo")
        .args(&["run", "--", "--config", "config.toml", "-s", "q", "-c", "q", "-p", "test"])
        .current_dir(get_project_dir())
        .output()
        .expect("Failed to execute command");

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
