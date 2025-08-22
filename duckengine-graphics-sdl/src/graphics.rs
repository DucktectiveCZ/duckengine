use duckengine_core::graphics::GraphicsBackend;
use log::debug;
use sdl2::version::version;

use crate::error::SdlGraphicsBackendError;

pub struct SdlGraphicsBackend {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl SdlGraphicsBackend {
    const DEFAULT_WINDOW_TITLE: &str = "Duck Engine";
    const DEFAULT_WINDOW_DIMENSONS: (u32, u32) = (800, 600);
}

impl GraphicsBackend for SdlGraphicsBackend {
    type Error = SdlGraphicsBackendError;

    fn new() -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let sdl = sdl2::init().map_err(SdlGraphicsBackendError::Sdl)?;
        let video_subsystem = sdl.video().map_err(SdlGraphicsBackendError::Sdl)?;
        let window = video_subsystem
            .window(
                Self::DEFAULT_WINDOW_TITLE,
                Self::DEFAULT_WINDOW_DIMENSONS.0,
                Self::DEFAULT_WINDOW_DIMENSONS.1,
            )
            .position_centered()
            .hidden()
            .build()
            .map_err(|e| SdlGraphicsBackendError::Sdl(e.to_string()))?;
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| SdlGraphicsBackendError::Sdl(e.to_string()))?;
        let mut event_pump = sdl
            .event_pump()
            .map_err(|e| SdlGraphicsBackendError::Sdl(e))?;

        Ok(Self {
            sdl,
            video_subsystem,
            canvas,
        })
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        let runtime_version = version();
        let platform = sdl2::get_platform();
        debug!("Starting the SDL graphics backend...");
        debug!(
            "Using SDL v{}.{}.{} on {}",
            runtime_version.major, runtime_version.minor, runtime_version.patch, platform
        );

        Ok(())
    }

    fn update(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn render(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn shutdown(&mut self) {
        debug!("Shutting down the SDL graphics backend...");
    }
}
