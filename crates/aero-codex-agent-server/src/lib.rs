#![forbid(unsafe_code)]
//! Local agent tool server scaffold.

use aero_codex_agent_core::*;
use serde_json::Value;

pub enum AgentServerRequest {
    ListTools,
    GetToolSpec { name: AgentToolName },
    Search { request: RetrievalRequest },
    Explain { codex_id: CodexId },
    Verify { codex_id: CodexId },
    Invoke { name: AgentToolName, input: Value },
    ReplayTrace { trace_id: TraceId },
}

pub struct AgentServerConfig {
    pub bind_addr: String,
    pub read_only: bool,
    pub trace_dir: Option<String>,
    pub request_timeout_ms: u64,
    pub max_input_bytes: usize,
}

impl Default for AgentServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:8765".to_owned(),
            read_only: true,
            trace_dir: Some("validation/traces".to_owned()),
            request_timeout_ms: 10_000,
            max_input_bytes: 1_000_000,
        }
    }
}

pub struct AgentServer {
    pub config: AgentServerConfig,
}

impl AgentServer {
    #[must_use]
    pub fn new(config: AgentServerConfig) -> Self {
        Self { config }
    }

    pub fn run_local(self) -> AgentResult<()> {
        // TODO: implement local transport: HTTP, JSON-RPC, or MCP-compatible adapter.
        Ok(())
    }
}
