//! Prepare and run a Windows window through the engine's public API.

#![cfg(windows)]

mod engine;
mod error;
mod runtime;

pub use engine::{Engine, EngineBuilder};
pub use error::Error;
