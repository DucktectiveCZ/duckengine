pub trait ScriptBackend {
    type Error: std::error::Error + Send + Sync + 'static;

    const SOURCE_FILE_EXT: &str;

    fn new() -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn start(&mut self) -> Result<(), Self::Error>;
    fn update(&mut self) -> Result<(), Self::Error>;
    fn shutdown(&mut self);
}
