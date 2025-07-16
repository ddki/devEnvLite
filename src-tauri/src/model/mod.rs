use anyhow::Result;

pub mod config;
pub mod env;
pub mod group;
pub mod settings;

mod env_config;
mod environment_variable;
mod variable_group;

pub use env_config::*;
pub use environment_variable::*;
pub use variable_group::*;

pub trait InfoEvent {
	fn apply(&self) -> Result<()>;
	fn check(&self) -> Result<()>;
}
