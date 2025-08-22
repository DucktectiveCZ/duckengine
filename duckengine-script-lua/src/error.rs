use duckengine_core::{graphics::GraphicsBackend, runtime::RuntimeError};

use crate::script::LuaScriptBackend;

#[derive(Debug, thiserror::Error)]
pub enum LuaScriptBackendError {}

impl<G: GraphicsBackend> From<LuaScriptBackendError> for RuntimeError<G, LuaScriptBackend> {
    fn from(err: LuaScriptBackendError) -> Self {
        RuntimeError::Script(err)
    }
}
