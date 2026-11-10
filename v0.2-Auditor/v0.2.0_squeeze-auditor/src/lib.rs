//! claude-squeeze: keep long output, repeated reads, oversized sessions and
//! idle MCP servers out of Claude's context. See README.md.

pub mod auditor;
pub mod cli;
pub mod clip;
pub mod compress;
pub mod config;
pub mod doctor;
pub mod guards;
pub mod hook;
pub mod install;
pub mod json;
pub mod ledger;
pub mod mcp;
pub mod mcp_server;
pub mod outline;
pub mod paths;
pub mod runner;
pub mod setup;
pub mod transcripts;
pub mod usage;

#[cfg(test)]
pub mod testutil;
