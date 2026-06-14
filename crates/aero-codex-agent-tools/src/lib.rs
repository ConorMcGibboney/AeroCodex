#![forbid(unsafe_code)]
//! Agent tool invocation engine.

use aero_codex_agent_core::*;
use serde_json::Value;
use std::collections::BTreeMap;

pub struct AgentToolTable {
    tools: BTreeMap<AgentToolName, Box<dyn AgentTool>>,
}

impl AgentToolTable {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
        }
    }

    pub fn insert<T>(&mut self, tool: T)
    where
        T: AgentTool + 'static,
    {
        self.tools.insert(tool.spec().name.clone(), Box::new(tool));
    }

    #[must_use]
    pub fn list(&self) -> Vec<&AgentToolSpec> {
        self.tools.values().map(|tool| tool.spec()).collect()
    }

    #[must_use]
    pub fn get(&self, name: &AgentToolName) -> Option<&dyn AgentTool> {
        self.tools.get(name).map(|tool| tool.as_ref())
    }

    pub fn invoke(
        &self,
        name: &AgentToolName,
        input: Value,
        context: &AgentInvocationContext,
    ) -> AgentResult<Value> {
        let tool = self.get(name).ok_or_else(|| {
            Box::new(AgentError {
                code: AgentErrorCode::UnknownTool,
                message: format!("Unknown AeroCodex agent tool: {name}"),
                codex_id: None,
                tool_name: Some(name.clone()),
                parameter: None,
                received_value: None,
                expected_condition: None,
                recoverability: Recoverability::SelectDifferentTool,
                suggested_tools: Vec::new(),
                suggested_questions: Vec::new(),
                evidence: None,
            })
        })?;
        tool.invoke_json(input, context)
    }
}

impl Default for AgentToolTable {
    fn default() -> Self {
        Self::new()
    }
}
