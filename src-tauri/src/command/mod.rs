use serde::{Deserialize, Serialize};

pub mod env_config;
pub mod environment_variable;
pub mod variable_group;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Success<T> {
	pub code: Option<String>,
	pub message: Option<String>,
	pub data: Option<T>,
}

impl<T> Success<T> {
	pub fn success(data: T) -> Self {
		Success {
			code: Some("200".to_string()),
			message: Some("Success".to_string()),
			data: Some(data),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fail {
	pub code: Option<String>,
	pub message: Option<String>,
}

impl Fail {
	pub fn fail(code: &str, message: &str) -> Self {
		Fail {
			code: Some(code.to_string()),
			message: Some(message.to_string()),
		}
	}

	pub fn fail_with_message(message: &str) -> Self {
		Fail {
			code: Some("500".to_string()),
			message: Some(message.to_string()),
		}
	}

	pub fn fail_default() -> Self {
		Fail {
			code: Some("500".to_string()),
			message: Some("Internal Server Error".to_string()),
		}
	}
}

pub type SResult<T> = Result<Success<T>, Fail>;
