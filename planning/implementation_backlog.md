# Agentic Implementation Backlog

## Now

- Create agent core type crate.
- Add agent metadata fields to evidence card parser.
- Hand-register one tool: `gasdyn.normal_shock.p2_over_p1`.
- Generate first JSON schema.
- Write first trace fixture.
- Add `agent list-tools` and `agent schema` CLI stubs.

## Next

- Implement registry builder.
- Generate CVTs from the normal shock evidence card.
- Add alias and symbol indexes.
- Add retrieval reasons.
- Implement `agent search` command.
- Implement `agent invoke` for the vertical slice.

## Later

- Add procedural macros.
- Add capability graph.
- Add explain and verify builders.
- Add trace replay.
- Add local server.
- Add optional embedding provider and vector search.
- Add WASM-compatible registry export.

## Design decisions to record as ADRs

- Whether the agent CLI is a separate crate or subcommands in the main CLI.
- Whether schemas are generated at build time, test time, or release time.
- Whether proc macros are required before v0.3.0-agent.
- Whether vector storage is JSON, binary, or database-backed.
- Whether the local server speaks HTTP, JSON-RPC, or an MCP-compatible protocol.
