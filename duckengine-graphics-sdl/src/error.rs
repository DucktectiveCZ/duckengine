use duckengine_core::{runtime::RuntimeError, script::ScriptBackend};

use crate::graphics::SdlGraphicsBackend;

#[derive(Debug, thiserror::Error)]
pub enum SdlGraphicsBackendError {
    #[error("SDL error")]
    Sdl(String),
}

impl<S: ScriptBackend> From<SdlGraphicsBackendError> for RuntimeError<SdlGraphicsBackend, S> {
    fn from(err: SdlGraphicsBackendError) -> Self {
        RuntimeError::Graphics(err)
    }
}
