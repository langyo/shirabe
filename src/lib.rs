//! shirabe — Lightweight headless browser automation.
//!
//! CDP (Chrome DevTools Protocol) engine + HTTP debug API.
//! Zero-config: finds Chrome automatically, launches headless, serves API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use shirabe::{start_debug_server, DebugServerConfig};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let cfg = DebugServerConfig {
//!     base_url: "about:blank".to_string(),
//!     proxy: None,
//! };
//! start_debug_server(cfg, 3001).await?;
//! # Ok(())
//! # }
//! ```

pub mod engine;
pub mod resolver;

pub use engine::{start_debug_server, DebugServerConfig};
