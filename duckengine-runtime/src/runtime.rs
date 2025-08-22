use duckengine_core::{
    game::Game, graphics::GraphicsBackend, runtime::RuntimeError, script::ScriptBackend,
};

pub struct Runtime<G: GraphicsBackend, S: ScriptBackend> {
    should_stop: bool,
    graphics: G,
    script: S,
}
impl<G: GraphicsBackend, S: ScriptBackend> Runtime<G, S>
where
    RuntimeError<G, S>: From<<S as ScriptBackend>::Error>,
    RuntimeError<G, S>: From<<G as GraphicsBackend>::Error>,
{
    pub fn new() -> Result<Self, RuntimeError<G, S>> {
        Ok(Self {
            should_stop: false,
            graphics: G::new().map_err(|err| RuntimeError::Graphics(err))?,
            script: S::new().map_err(|err| RuntimeError::Script(err))?,
        })
    }

    pub fn run(&mut self, _game: Game) -> Result<(), RuntimeError<G, S>> {
        self.script.start()?;
        self.graphics.start()?;

        self.should_stop = false;

        while !self.should_stop {
            self.update()?;
            self.graphics.render()?;
        }

        self.shutdown();

        Ok(())
    }

    fn update(&mut self) -> Result<(), RuntimeError<G, S>> {
        self.script.update()?;
        self.graphics.update()?;
        Ok(())
    }

    fn shutdown(&mut self) {
        self.script.shutdown();
        self.graphics.shutdown();
    }
}
