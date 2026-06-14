#![forbid(unsafe_code)]
//! Agent trace writer, reader, replay, diff, and summary helpers.

use aero_codex_agent_core::*;
use std::path::Path;

pub struct TraceWriter;
pub struct TraceReader;
pub struct TraceReplayer;

impl TraceWriter {
    pub fn write_json(path: &Path, trace: &AgentTrace) -> Result<(), std::io::Error> {
        let bytes = serde_json::to_vec_pretty(trace).expect("AgentTrace should serialize");
        std::fs::write(path, bytes)
    }
}

impl TraceReader {
    pub fn read_json(path: &Path) -> Result<AgentTrace, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        let trace = serde_json::from_slice(&bytes)?;
        Ok(trace)
    }
}

impl TraceReplayer {
    pub fn replay(_trace: &AgentTrace) -> AgentResult<()> {
        // TODO: locate tool by exact tool name and Codex ID, invoke with normalized input,
        // then compare to recorded output using tool-defined tolerances.
        Ok(())
    }
}
