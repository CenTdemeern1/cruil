#![doc = include_str!("../README.md")]
//!
//! # Example
//! From `examples/player_one.rs`:
//! ```no_run
#![doc = include_str!("../examples/player_one.rs")]
//! ```

// Private modules
mod backend;

// Public modules
pub mod keyboard;
pub mod mouse;

// Re-exported modules
mod constants;
pub use constants::*;
mod device_kind;
pub use device_kind::*;
mod threaded_reader;
pub use threaded_reader::*;
mod input_state;
pub use input_state::*;
mod input_device;
pub use input_device::*;
mod readable_device;
pub use readable_device::*;
mod cruil;
pub use cruil::*;
mod error;
pub use error::*;
