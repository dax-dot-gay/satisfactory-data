use thiserror::Error;

/// Library-specific error type
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Unexpected error: {0:?}")]
    Unknown(#[from] anyhow::Error),

    #[error("Invalid ID format: {0}")]
    InvalidId(String),

    #[error("IOError (std::io): {0:?}")]
    Io(#[from] std::io::Error),

    #[error("Archive error (zip::*): {0:?}")]
    Archive(#[from] zip::result::ZipError),

    #[error("JSON de/serialization error (serde::*): {0:?}")]
    Json(#[from] serde_json::Error),

    #[error("Request error (reqwest::*): {0:?}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to load registry from {location}: {error:?}")]
    RegistryLoadFailure {
        location: String,
        error: Box<Self>
    },

    #[error("Failed to persist registry: {0}")]
    RegistryPersistError(String),

    #[error("This registry has not been persisted and is thus immutable! Run <registry>.persist(...) to create a mutable registry.")]
    ImmutableRegistry,

    #[error("Attempted to extract RegistryItem [{0}] as the wrong type.")]
    RegistryExtractMismatch(String)
}

#[allow(missing_docs)]
impl Error {
    pub fn invalid_id(id: impl Into<String>) -> Self {
        Self::InvalidId(id.into())
    }

    pub fn registry_load(location: impl Into<String>, error: impl Into<Self>) -> Self {
        Self::RegistryLoadFailure { location: location.into(), error: Box::new(error.into()) }
    }

    pub fn registry_persist(reason: impl Into<String>) -> Self {
        Self::RegistryPersistError(reason.into())
    }

    pub fn registry_extract_mismatch(id: impl Into<String>) -> Self {
        Self::RegistryExtractMismatch(id.into())
    }
}

#[allow(unused)]
pub type Result<T> = std::result::Result<T, Error>;
