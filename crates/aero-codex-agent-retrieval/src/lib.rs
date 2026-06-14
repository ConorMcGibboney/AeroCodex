#![forbid(unsafe_code)]
//! Hybrid retrieval engine.

use aero_codex_agent_core::*;

pub trait Retriever {
    fn search(&self, request: RetrievalRequest) -> AgentResult<Vec<RetrievalResult>>;
}

pub struct HybridRetriever {
    // TODO: lexical index, alias index, symbol index, metadata index, vector index, capability graph.
}

impl HybridRetriever {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for HybridRetriever {
    fn default() -> Self {
        Self::new()
    }
}

impl Retriever for HybridRetriever {
    fn search(&self, _request: RetrievalRequest) -> AgentResult<Vec<RetrievalResult>> {
        // TODO: implement exact ID, symbol, alias, metadata, vector, and graph expansion.
        Ok(Vec::new())
    }
}
