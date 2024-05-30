#[derive(Debug, thiserror::Error)]
pub enum AppError {
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Other(#[from] anyhow::Error),

	#[error("Application error: {0}")]
	Custom(String),
}

impl serde::Serialize for AppError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		serializer.serialize_str(self.to_string().as_ref())
	}
}
