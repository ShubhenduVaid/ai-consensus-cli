use crate::{ToolConfig, tools::ToolManager, Result};

pub struct ConsensusEngine;

impl ConsensusEngine {
    pub async fn get_consensus(consensus_tool: &ToolConfig, responses: Vec<String>, _prompt: &str) -> Result<String> {
        let consensus_prompt = format!(
            "Analyze these AI responses and provide a clear, concise consensus answer. Be direct and avoid meta-commentary about the analysis process:\n\n{}",
            responses.iter().enumerate()
                .map(|(i, r)| format!("Response {}: {}", i + 1, r))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        
        ToolManager::run_tool(consensus_tool, &consensus_prompt).await
    }
}
