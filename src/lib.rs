#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod browser;
mod error;
mod generated;
mod header;
mod language;
mod platform;
mod profile;
mod user_agents;

pub use browser::Browser;
pub use error::{Error, Result};
pub use header::Header;
pub use platform::Platform;
pub use profile::Profile;
pub use user_agents::UserAgents;
