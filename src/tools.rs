use crate::{Config, ToolConfig, ui, Validator, CliError, Result, constants::*};
use log::{info, warn, error};
use std::time::Duration;
use std::process::Stdio;
use tokio::task;

pub struct ToolManager;

impl ToolManager {
    pub fn is_available(tool_config: &ToolConfig) -> bool {
        if Validator::validate_command(&tool_config.command).is_err() {
            return false;
        }
        
        match std::process::Command::new("which")
            .arg(&tool_config.command)
            .output() 
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    pub fn check_availability(solvers: &[String], config: &Config) -> (Vec<(String, ToolConfig)>, Vec<String>) {
        let mut available_solvers = Vec::new();
        let mut unavailable_tools = Vec::new();
        
        for solver_name in solvers {
            let tool_config = &config.tools[solver_name];
            if Self::is_available(tool_config) {
                available_solvers.push((solver_name.clone(), tool_config.clone()));
            } else {
                unavailable_tools.push(solver_name.clone());
            }
        }
        
        (available_solvers, unavailable_tools)
    }

    pub async fn run_tool(tool_config: &ToolConfig, prompt: &str) -> Result<String> {
        let sanitized_args = Validator::sanitize_args(&tool_config.args, prompt)?;
        
        let mut cmd = tokio::process::Command::new(&tool_config.command);
        
        for arg in &sanitized_args {
            cmd.arg(arg);
        }
        
        cmd.stdin(Stdio::null())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .kill_on_drop(true);
        
        #[cfg(unix)]
        {
            unsafe {
                cmd.pre_exec(|| {
                    let limit = libc::rlimit {
                        rlim_cur: MEMORY_LIMIT_MB * 1024 * 1024,
                        rlim_max: MEMORY_LIMIT_MB * 1024 * 1024,
                    };
                    libc::setrlimit(libc::RLIMIT_AS, &limit);
                    
                    let cpu_limit = libc::rlimit {
                        rlim_cur: CPU_LIMIT_SECS,
                        rlim_max: CPU_LIMIT_SECS,
                    };
                    libc::setrlimit(libc::RLIMIT_CPU, &cpu_limit);
                    Ok(())
                });
            }
        }
        
        info!("Running tool: {}", tool_config.name);
        
        let output = tokio::time::timeout(
            Duration::from_secs(TOOL_TIMEOUT_SECS),
            cmd.output()
        ).await
        .map_err(|_| CliError::ToolTimeout { 
            tool: tool_config.name.clone(), 
            timeout: TOOL_TIMEOUT_SECS 
        })?
        .map_err(|e| CliError::ConfigError { 
            message: format!("Failed to execute tool '{}': {}", tool_config.name, e)
        })?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined_output = format!("{}{}", stdout, stderr);
        
        Ok(combined_output)
    }

    pub async fn run_solvers(available_solvers: &[(String, ToolConfig)], prompt: &str) -> Result<Vec<String>> {
        let solver_count = available_solvers.len();
        
        if solver_count == 1 {
            println!("🤖 Running 1 solver...");
        } else {
            ui::show_progress_start(solver_count);
        }
        
        let start_time = std::time::Instant::now();
        
        let mut tasks = Vec::new();
        for (solver_idx, (solver_name, tool_config)) in available_solvers.iter().enumerate() {
            let tool_config = tool_config.clone();
            let prompt = prompt.to_string();
            let solver_name = solver_name.clone();
            
            tasks.push(task::spawn(async move {
                let result = Self::run_tool(&tool_config, &prompt).await;
                (solver_idx, solver_name, result)
            }));
        }
        
        let mut responses = Vec::new();
        let mut failed_tools = Vec::new();
        
        for task in tasks {
            match task.await.map_err(|e| CliError::ConfigError { 
                message: format!("Task join error: {}", e)
            })? {
                (_solver_idx, solver_name, Ok(response)) => {
                    if Validator::is_authentication_error(&response) {
                        ui::show_failure();
                        warn!("Authentication error for tool: {}", solver_name);
                        failed_tools.push(solver_name);
                    } else {
                        ui::show_success();
                        info!("Tool {} completed successfully", solver_name);
                        responses.push(response);
                    }
                },
                (_solver_idx, solver_name, Err(e)) => {
                    ui::show_failure();
                    error!("Tool {} failed: {}", solver_name, e);
                    failed_tools.push(solver_name);
                }
            }
        }
        
        let solver_time = start_time.elapsed();
        ui::show_timing(solver_time.as_secs_f32());
        
        if !failed_tools.is_empty() {
            warn!("Failed tools: {}", failed_tools.join(", "));
        }
        
        if responses.is_empty() {
            return Err(CliError::AllSolversFailed);
        }
        
        info!("Successfully collected {} responses", responses.len());
        Ok(responses)
    }
}
