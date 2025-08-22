use std::io;
use std::path::PathBuf;
use std::result::Result as StdResult;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("OS error: {0}")]
    OsError(&'static str),
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    #[error("Toml deserialize error in '{file}': {err}")]
    TomlDeError{ err: toml::de::Error, file: PathBuf },
    #[error("Toml serialize error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
}

pub type LoaderResult<T> = StdResult<T, LoaderError>;
