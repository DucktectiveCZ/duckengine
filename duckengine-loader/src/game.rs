use std::{collections::HashMap, fs::read_to_string, io, path::PathBuf, str::FromStr};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::loader::{LoaderError, LoaderResult, types::Type};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMeta {
    pub name: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub entry: String, // Game's entry point scene
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub meta: GameMeta,   // Metadata
    pub game: GameConfig, // Game config
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityInfo {
    pub children: Vec<String>,            // The entity's child entities
    pub variables: HashMap<String, Type>, // The entity's variables
    pub requires: Vec<String>,            // The entities the parent must have
}

pub struct Entity {
    pub info: EntityInfo,   // .toml
    pub code_path: PathBuf, // .lua
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneInfo {
    pub children: Vec<String>,            // The entity's child entities
    pub variables: HashMap<String, Type>, // The entity's variables
}

pub struct Scene {
    pub info: SceneInfo, // .toml
    pub code: PathBuf,   // .lua
}

pub struct Game {
    pub info: GameInfo,                    // The game config/metadata
    pub assets: HashMap<String, PathBuf>,  // The asset keys and their paths
    pub entities: HashMap<String, Entity>, // The entities defined
    pub scenes: HashMap<String, Scene>,    // The scenes defined
    pub common: PathBuf,                   // Common code shared by all the scripts
}

impl Game {
    const INFO_FILE: &str = "game.toml";
    const ASSETS_PATH: &str = "assets";
    const ENTITIES_PATH: &str = "entities";
    const SCENES_PATH: &str = "scenes";

    pub fn load_game_info(path: &PathBuf) -> LoaderResult<GameInfo> {
        let toml_str = read_to_string(path)?;
        Ok(toml::from_str(&toml_str)?)
    }

    pub fn load_assets(path: &PathBuf) -> LoaderResult<HashMap<String, PathBuf>> {
        let assets_path = path.join(Self::ASSETS_PATH);

        let mut assets = HashMap::new();
        for entry_res in assets_path.read_dir()? {
            let entry = entry_res?;
            if entry.file_type()?.is_file() {
                let path = entry.path();
                let key = path
                    .file_name()
                    .map(|k| {
                        k.to_str()
                            .expect("Failed to convert a OsString to a &str")
                            .to_string()
                    })
                    .ok_or_else(|| LoaderError::InvalidPath(path.clone()))?;
                assets.insert(key, path);
            }
        }

        Ok(assets)
    }

    pub fn load_entities(base_path: &PathBuf) -> LoaderResult<HashMap<String, Entity>> {
        let entities_path = base_path.join(Self::ENTITIES_PATH);

        let mut entities = HashMap::new();
        for entry in entities_path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
                continue;
            };

            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };

            let (info_path, code_path) = match ext {
                "toml" => (path.clone(), PathBuf::from(format!("{stem}.lua"))),
                "lua" => (PathBuf::from(format!("{stem}.toml")), path.clone()),
                _ => continue,
            };

            let info_toml = read_to_string(info_path)?;
            let info = toml::from_str(&info_toml)?;

            entities.insert(stem.to_string(), Entity { info, code_path });
        }

        Ok(entities)
    }

    pub fn load_scenes(base_path: &PathBuf) -> LoaderResult<HashMap<String, Scene>> {
        let entities_path = base_path.join(Self::SCENES_PATH);

        let mut entities = HashMap::new();
        for entry in entities_path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
                continue;
            };

            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };

            let (info_path, code_path) = match ext {
                "toml" => (path.clone(), PathBuf::from(format!("{stem}.lua"))),
                "lua" => (PathBuf::from(format!("{stem}.toml")), path.clone()),
                _ => continue,
            };

            let info_toml = read_to_string(info_path)?;
            let info = toml::from_str(&info_toml)?;

            entities.insert(stem.to_string(), Entity { info, code_path });
        }

        Ok(entities)
    }

    pub fn load_folder(path: PathBuf) -> LoaderResult<Self> {
        let info = Self::load_game_info(&path)?;
        let assets = Self::load_assets(&path)?;
        let entities = Self::load_entities(&path)?;
        let scenes = Self::load_scenes(&path)?;

        todo!()
    }
}
