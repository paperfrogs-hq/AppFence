//! AppFence Core
//!
//! Shared types, utilities, and application identity logic.

pub mod app_id;
pub mod types;
pub mod error;

pub use app_id::AppId;
pub use types::*;
pub use error::*;
