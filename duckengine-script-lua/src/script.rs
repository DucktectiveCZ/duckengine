use duckengine_core::script::ScriptBackend;
use log::debug;

use crate::error::LuaScriptBackendError;

#[derive(Debug)]
pub struct LuaScriptBackend;

impl ScriptBackend for LuaScriptBackend {
    type Error = LuaScriptBackendError;

    const SOURCE_FILE_EXT: &str = "lua";

    fn new() -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        debug!("Starting the Lua script backend...");
        Ok(())
    }

    fn update(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn shutdown(&mut self) {
        debug!("Shutting down the Lua script backend...")
    }
}
