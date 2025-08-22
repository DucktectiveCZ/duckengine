use std::fmt::Debug;

pub trait GraphicsBackend {
    type Error: std::error::Error + Debug + Send + Sync + 'static;

    fn new() -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn start(&mut self) -> Result<(), Self::Error>;
    fn update(&mut self) -> Result<(), Self::Error>;
    fn render(&mut self) -> Result<(), Self::Error>;
    fn shutdown(&mut self);
}
