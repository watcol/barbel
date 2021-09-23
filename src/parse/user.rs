use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use toml::Value;

#[derive(Deserialize, Debug)]
pub struct Entry {
    #[serde(flatten)]
    global: HashMap<String, Value>,
    #[serde(default)]
    include: Vec<PathBuf>,
    #[serde(default)]
    renderer: Vec<Renderer>,
}

#[derive(Deserialize, Debug)]
struct Renderer {
    source: String,
    output: Option<String>,
    #[serde(default)]
    include: Vec<PathBuf>,
    #[serde(flatten)]
    table: HashMap<String, Value>,
}

pub fn parse_entry<P: AsRef<Path>>(path: P) -> anyhow::Result<Entry> {
    let entry = super::read(path)?;
    Ok(toml::from_str(&entry)?)
}
