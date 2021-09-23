use anyhow::Context;
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use toml::Value;

#[derive(Clone, Debug, Deserialize)]
struct TomlEntry {
    #[serde(default)]
    renderer: Vec<TomlRenderer>,
    #[serde(flatten)]
    config: TomlConfig,
}

impl TomlEntry {
    fn into_entry(self, dir: &Path) -> anyhow::Result<Entry> {
        Ok(Entry {
            renderer: self
                .renderer
                .into_iter()
                .map(|r| r.into_renderer(dir))
                .collect::<Result<Vec<_>, _>>()?,
            config: self.config.into_config(dir)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct TomlRenderer {
    source: String,
    #[serde(flatten)]
    config: TomlConfig,
}

impl TomlRenderer {
    fn into_renderer(self, dir: &Path) -> anyhow::Result<Renderer> {
        Ok(Renderer {
            source: self.source,
            config: self.config.into_config(dir)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct TomlConfig {
    #[serde(default)]
    include: Vec<PathBuf>,
    #[serde(flatten)]
    table: HashMap<String, Value>,
}

impl TomlConfig {
    fn into_config(self, dir: &Path) -> anyhow::Result<HashMap<String, Value>> {
        let base_dir =
            dir.parent().context("Connot specify root directory.")?;
        let mut map = HashMap::new();
        for path in self.include {
            let path = if path.is_absolute() {
                path
            } else {
                base_dir.join(path)
            };
            let config = super::read(&path)?;
            let config: TomlConfig = toml::from_str(&config)?;
            map.extend(config.into_config(&path)?);
        }
        map.extend(self.table);
        Ok(map)
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub renderer: Vec<Renderer>,
    pub config: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct Renderer {
    pub source: String,
    pub config: HashMap<String, Value>,
}

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<Entry> {
    let s = super::read(&path)?;
    let toml_entry: TomlEntry = toml::from_str(&s)?;
    toml_entry.into_entry(path.as_ref())
}
