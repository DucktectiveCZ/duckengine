use std::io;
use std::path::PathBuf;
use std::result::Result as StdResult;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("Toml deserialize error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("Toml serialize error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
}

pub type LoaderResult<T> = StdResult<T, LoaderError>;
