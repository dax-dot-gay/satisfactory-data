use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected error: {0:?}")]
    Unknown(#[from] anyhow::Error),

    #[error("Invalid ID format: {0}")]
    InvalidId(String),
}

impl Error {
    pub fn invalid_id(id: impl Into<String>) -> Self {
        Self::InvalidId(id.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
