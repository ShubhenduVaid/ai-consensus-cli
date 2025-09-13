use clap::Parser;
use log::{info, warn};

use ai_consensus_cli::{Config, ToolManager, ConsensusEngine, Validator, ui, Result};

#[derive(Parser)]
#[command(name = "ai-co")]
#[command(about = "Orchestrate multiple AI CLIs with consensus")]
struct Cli {
    #[arg(short, long, value_delimiter = ',')]
    #[arg(help = "AI tools to solve the problem (comma-separated)")]
    solvers: Vec<String>,
    
    #[arg(short, long)]
    #[arg(help = "AI tool to provide consensus on solver responses")]
    consensus: String,
    
    #[arg(short, long)]
    #[arg(help = "Question or problem to solve")]
    prompt: String,
    
    #[arg(long, default_value = "config.toml")]
    #[arg(help = "Path to configuration file")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    info!("Starting AI Consensus CLI");
    
    // Load configuration with secure path validation
    let config = match Config::load(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration Error: {}", e);
            eprintln!("\nTo fix this:");
            eprintln!("1. Ensure config.toml exists at ~/.config/ai-consensus-cli/config.toml, or");
            eprintln!("2. Copy config.toml to your current directory, or");
            eprintln!("3. Use --config /path/to/config.toml to specify the location");
            eprintln!("\nExample config.toml can be found at:");
            eprintln!("https://github.com/your-repo/ai-consensus-cli/blob/main/config.toml");
            return Err(e);
        }
    };
    
    // Show help if requested (simplified check)
    if cli.solvers.is_empty() || cli.consensus.is_empty() || cli.prompt.is_empty() {
        println!("{}", config.generate_help_text());
        std::process::exit(0);
    }
    
    // Validate tools and prompt with security checks
    if let Err(e) = Validator::validate_tools(&cli.solvers, &cli.consensus, &config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    if let Err(e) = Validator::sanitize_prompt(&cli.prompt) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    // Check tool availability
    let (available_solvers, unavailable_tools) = ToolManager::check_availability(&cli.solvers, &config);
    
    if !config.tools.contains_key(&cli.consensus) {
        eprintln!("Error: ToolNotFound - consensus tool '{}' not found", cli.consensus);
        std::process::exit(1);
    }
    
    let consensus_tool = &config.tools[&cli.consensus];
    if !ToolManager::is_available(consensus_tool) {
        eprintln!("Error: ToolNotFound - consensus tool '{}' not available", cli.consensus);
        std::process::exit(1);
    }
    
    if !unavailable_tools.is_empty() {
        warn!("Unavailable tools: {}", unavailable_tools.join(", "));
        println!("❌ Unavailable tools: {} (skipping)", unavailable_tools.join(", "));
    }
    
    if available_solvers.is_empty() {
        eprintln!("Error: All solvers failed - no available tools");
        std::process::exit(1);
    }
    
    // Run solvers
    let responses = ToolManager::run_solvers(&available_solvers, &cli.prompt).await?;
    
    // Get consensus
    ui::show_consensus_start();
    
    let consensus_start = std::time::Instant::now();
    let consensus = ConsensusEngine::get_consensus(consensus_tool, responses, &cli.prompt).await?;
    let consensus_time = consensus_start.elapsed();
    
    ui::show_consensus_complete(consensus_time.as_secs_f32());
    
    // Clean output - remove ANSI codes and extra formatting
    let clean_consensus = Validator::strip_ansi_codes(&consensus);
    println!("{}", clean_consensus.trim());
    
    info!("AI Consensus CLI completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test that CLI parsing works with the new structure
        let cli = Cli::parse_from(&["ai-co", "-s", "test", "-c", "test", "-p", "test prompt"]);
        assert_eq!(cli.solvers, vec!["test"]);
        assert_eq!(cli.consensus, "test");
        assert_eq!(cli.prompt, "test prompt");
    }
}
