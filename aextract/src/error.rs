use std::{
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Error, Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DocsError {
    #[error("Failed to read docs file {path:?}: {reason}")]
    FailedRead { path: PathBuf, reason: String },

    #[error("Invalid docs file format: {reason}")]
    InvalidFormat { reason: String },
}

impl DocsError {
    pub fn failed_read(path: impl AsRef<Path>, error: io::Error) -> Self {
        Self::FailedRead {
            path: path.as_ref().to_path_buf(),
            reason: error.to_string(),
        }
    }

    pub fn invalid_format(reason: impl AsRef<str>) -> Self {
        Self::InvalidFormat {
            reason: reason.as_ref().to_string(),
        }
    }
}

#[derive(Error, Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "category", rename_all = "snake_case")]
pub enum CommonError {
    #[error(transparent)]
    Docs {
        #[serde(flatten)]
        error: DocsError,
    },
}

impl From<DocsError> for CommonError {
    fn from(value: DocsError) -> Self {
        Self::Docs { error: value }
    }
}

pub type Result<T> = std::result::Result<T, CommonError>;
