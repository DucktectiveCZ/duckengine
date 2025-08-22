use std::{collections::HashMap, path::PathBuf};

use semver::Version;

#[derive(Debug)]
pub enum Type {
    Int,
    Num,
    Str,
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Num(f32),
    Str(String),
    Table(HashMap<String, Value>),
}

#[derive(Debug)]
pub struct GameMeta {
    pub name: String,
    pub description: Option<String>,
    pub author: String,
    pub license: Option<String>,
    pub version: Version,
}

#[derive(Debug)]
pub struct GameConfig {
    pub entry: String, // Game's entry point scene
}

#[derive(Debug)]
pub struct GameInfo {
    pub meta: GameMeta,   // Metadata
    pub game: GameConfig, // Game config
}

#[derive(Debug)]
pub struct EntityInfo {
    pub children: HashMap<String, HashMap<String, Value>>, // The entity's child entities
    pub variables: HashMap<String, Type>,                  // The entity's variables
    pub requires: Option<Vec<String>>,                     // The entities the parent must have
}

#[derive(Debug)]
pub struct Entity {
    pub info: EntityInfo,
    pub code_path: PathBuf,
}

#[derive(Debug)]
pub struct SceneInfo {
    pub children: HashMap<String, HashMap<String, Value>>, // The scene's child entities
}

#[derive(Debug)]
pub struct Scene {
    pub info: SceneInfo,
    pub code_path: PathBuf,
}

#[derive(Debug)]
pub struct Game {
    pub info: GameInfo,                    // The game config/metadata
    pub assets: HashMap<String, PathBuf>,  // The asset keys and their paths
    pub entities: HashMap<String, Entity>, // The entities defined
    pub scenes: HashMap<String, Scene>,    // The scenes defined
    pub common: Vec<PathBuf>,              // Common code shared by all the scripts
}
