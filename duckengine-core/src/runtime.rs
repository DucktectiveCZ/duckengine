use crate::{graphics::GraphicsBackend, script::ScriptBackend};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError<G: GraphicsBackend, S: ScriptBackend> {
    #[error("Graphics error: {0}")]
    Graphics(G::Error),
    #[error("Script error: {0}")]
    Script(S::Error),
}
