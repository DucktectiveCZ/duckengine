// The convert functons are AI-generated because i really don't have time for this rn. I'll make my
// own impl later. I sware I'm not a vibe coder

use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use semver::Version;
use serde::Deserialize;

use crate::error::{LoaderError, LoaderResult};

use duckengine_core::{game as core, script::ScriptBackend};

#[derive(Debug, Deserialize)]
pub enum Type {
    Int,
    Num,
    Str,
}

impl Type {
    const fn convert(&self) -> core::Type {
        use Type::*;

        match self {
            Int => core::Type::Int,
            Num => core::Type::Num,
            Str => core::Type::Str,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GameMeta {
    pub name: String,
    pub description: Option<String>,
    pub author: String,
    pub license: Option<String>,
    pub version: Version,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub entry: String, // Game's entry point scene
}

#[derive(Debug, Deserialize)]
pub struct GameInfo {
    pub meta: GameMeta,   // Metadata
    pub game: GameConfig, // Game config
}

#[derive(Debug, Deserialize)]
pub struct EntityInfo {
    pub children: HashMap<String, HashMap<String, toml::Value>>, // The entity's child entities pub variables: HashMap<String, Type>
    pub variables: HashMap<String, Type>,                        // The entity's variables
    pub requires: Option<Vec<String>>, // The entities the parent must have
}

#[derive(Debug)]
pub struct Entity {
    pub info: EntityInfo,   // .toml
    pub code_path: PathBuf, // .lua
}

#[derive(Debug, Deserialize)]
pub struct SceneInfo {
    pub children: HashMap<String, HashMap<String, toml::Value>>, // The scene's child entities
}

#[derive(Debug)]
pub struct Scene {
    pub info: SceneInfo,    // .toml
    pub code_path: PathBuf, // .lua
}

#[derive(Debug)]
pub struct Game {
    pub info: GameInfo,                    // The game config/metadata
    pub assets: HashMap<String, PathBuf>,  // The asset keys and their paths
    pub entities: HashMap<String, Entity>, // The entities defined
    pub scenes: HashMap<String, Scene>,    // The scenes defined
    pub common: Vec<PathBuf>,              // Common code shared by all the scripts
}

const INFO_FILE: &str = "game.toml";
const ASSETS_PATH: &str = "assets";
const ENTITIES_PATH: &str = "entities";
const SCENES_PATH: &str = "scenes";
const COMMON_PATH: &str = "common";

fn load_game_info(base_path: &PathBuf) -> LoaderResult<GameInfo> {
    let path = base_path.join(INFO_FILE);
    if !path.exists() {
        return Err(LoaderError::FileNotFound(path));
    }
    let toml_str = read_to_string(&path)?;
    Ok(toml::from_str(&toml_str).map_err(|e| LoaderError::TomlDeError { err: e, file: path })?)
}

fn load_assets(path: &PathBuf) -> LoaderResult<HashMap<String, PathBuf>> {
    let path = path.join(ASSETS_PATH);
    if !path.exists() {
        return Err(LoaderError::FileNotFound(path));
    }

    let mut assets = HashMap::new();
    for entry_res in path.read_dir()? {
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

fn load_entities(
    base_path: &PathBuf,
    info_ext: &str,
    script_ext: &str,
) -> LoaderResult<HashMap<String, Entity>> {
    let path = base_path.join(ENTITIES_PATH);
    if !path.exists() {
        return Err(LoaderError::FileNotFound(path));
    }

    let mut entities = HashMap::new();
    for entry in path.read_dir()? {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            return Err(LoaderError::OsError(
                "A file stem couldn't be converted to a string.",
            ));
        };

        let (info_path, code_path) = if ext == info_ext {
            (path.clone(), path.with_extension(script_ext))
        } else if ext == script_ext {
            (path.with_extension(info_ext), path.clone())
        } else {
            continue;
        };

        if !info_path.exists() {
            return Err(LoaderError::FileNotFound(info_path));
        }
        let info_toml = read_to_string(&info_path)?;
        let info = toml::from_str(&info_toml).map_err(|e| LoaderError::TomlDeError {
            err: e,
            file: info_path,
        })?;

        entities.insert(stem.to_string(), Entity { info, code_path });
    }

    Ok(entities)
}

fn load_scenes(
    base_path: &PathBuf,
    info_ext: &str,
    script_ext: &str,
) -> LoaderResult<HashMap<String, Scene>> {
    let path = base_path.join(SCENES_PATH);
    if !path.exists() {
        return Err(LoaderError::FileNotFound(path));
    }

    let mut scenes = HashMap::new();
    for entry in path.read_dir()? {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            return Err(LoaderError::OsError(
                "A file stem couldn't be converted to a string.",
            ));
        };

        let (info_path, code_path) = if ext == info_ext {
            (path.clone(), path.with_extension(script_ext))
        } else if ext == script_ext {
            (path.with_extension(info_ext), path.clone())
        } else {
            continue;
        };

        if !info_path.exists() {
            return Err(LoaderError::FileNotFound(info_path));
        }
        let info_toml = read_to_string(&info_path)?;
        let info = toml::from_str(&info_toml).map_err(|e| LoaderError::TomlDeError {
            err: e,
            file: info_path,
        })?;

        scenes.insert(stem.to_string(), Scene { info, code_path });
    }

    Ok(scenes)
}

fn load_common(base_path: &PathBuf, script_ext: &str) -> LoaderResult<Vec<PathBuf>> {
    let base_path = base_path.join(COMMON_PATH);
    if !base_path.exists() {
        return Err(LoaderError::FileNotFound(base_path));
    }

    let mut common = Vec::new();
    for entry in base_path.read_dir()? {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();

        if !path.is_file() || path.extension().map(|x| x.to_str()) != Some(Some(script_ext)) {
            continue;
        }

        common.push(entry.path());
    }

    Ok(common)
}

fn load_intermediate(path: &PathBuf, info_ext: &str, script_ext: &str) -> LoaderResult<Game> {
    let info = load_game_info(&path)?;
    let assets = load_assets(&path)?;
    let entities = load_entities(&path, info_ext, script_ext)?;
    let scenes = load_scenes(&path, info_ext, script_ext)?;
    let common = load_common(&path, script_ext)?;

    Ok(Game {
        info,
        assets,
        entities,
        scenes,
        common,
    })
}

fn convert_value(val: toml::Value) -> core::Value {
    match val {
        toml::Value::Integer(i) => core::Value::Int(i),
        toml::Value::Float(f) => core::Value::Num(f as f32),
        toml::Value::String(s) => core::Value::Str(s),
        toml::Value::Table(t) => {
            let mut map = HashMap::new();
            for (k, v) in t {
                map.insert(k, convert_value(v));
            }
            core::Value::Table(map)
        }
        toml::Value::Array(arr) => {
            let mut map = HashMap::new();
            for (i, v) in arr.into_iter().enumerate() {
                map.insert(i.to_string(), convert_value(v));
            }
            core::Value::Table(map)
        }
        toml::Value::Boolean(b) => core::Value::Str(b.to_string()),
        toml::Value::Datetime(dt) => core::Value::Str(dt.to_string()),
    }
}

fn convert_entity_info(info: EntityInfo) -> core::EntityInfo {
    let children = info
        .children
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|(k2, v2)| (k2, convert_value(v2)))
                    .collect(),
            )
        })
        .collect();

    core::EntityInfo {
        children,
        variables: info
            .variables
            .iter()
            .map(|v| (v.0.clone(), v.1.convert()))
            .collect(),
        requires: info.requires,
    }
}

fn convert_scene_info(info: SceneInfo) -> core::SceneInfo {
    let children = info
        .children
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|(k2, v2)| (k2, convert_value(v2)))
                    .collect(),
            )
        })
        .collect();

    core::SceneInfo { children }
}

pub fn convert_game(game: Game) -> core::Game {
    core::Game {
        info: core::GameInfo {
            meta: core::GameMeta {
                name: game.info.meta.name,
                description: game.info.meta.description,
                author: game.info.meta.author,
                license: game.info.meta.license,
                version: game.info.meta.version,
            },
            game: core::GameConfig {
                entry: game.info.game.entry,
            },
        },
        assets: game.assets,
        entities: game
            .entities
            .into_iter()
            .map(|(k, e)| {
                (
                    k,
                    core::Entity {
                        info: convert_entity_info(e.info),
                        code_path: e.code_path,
                    },
                )
            })
            .collect(),
        scenes: game
            .scenes
            .into_iter()
            .map(|(k, s)| {
                (
                    k,
                    core::Scene {
                        info: convert_scene_info(s.info),
                        code_path: s.code_path,
                    },
                )
            })
            .collect(),
        common: game.common,
    }
}

pub fn load<S: ScriptBackend>(
    path: &PathBuf,
    info_ext: &str,
) -> LoaderResult<duckengine_core::game::Game> {
    let intermediate = load_intermediate(path, info_ext, S::SOURCE_FILE_EXT)?;
    Ok(convert_game(intermediate))
}
