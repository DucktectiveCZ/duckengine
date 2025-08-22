use duckengine_graphics_sdl::graphics::SdlGraphicsBackend;
use duckengine_runtime::runtime::Runtime;
use duckengine_script_lua::script::LuaScriptBackend;
use log::{debug, error, trace};
use std::{path::PathBuf, process::exit};

use clap::Parser;

type G = SdlGraphicsBackend;
type S = LuaScriptBackend;

#[derive(clap_derive::Parser)]
struct Cli {
    #[arg(short, long)]
    pub run: Option<PathBuf>,
}

fn run_game(game_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Loading game at {:?}", game_path);
    let mut runtime = Runtime::<G, S>::new()?;
    trace!("Created the runtime");
    let game = duckengine_loader_folder::game::load::<S>(game_path, "toml")?;
    trace!(
        "Loaded the game {} v{} by {}",
        game.info.meta.name, game.info.meta.version, game.info.meta.author
    );
    runtime.run(game)?;

    Ok(())
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    if let Some(game_path) = cli.run {
        if let Err(err) = run_game(&game_path) {
            error!("Error: {err}");
            exit(1);
        }
    }
}
