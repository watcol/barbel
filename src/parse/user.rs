use anyhow::Context;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};
use toml::Value;

use super::uri::Uri;

#[derive(Clone, Debug, Deserialize)]
struct TomlEntry {
    #[serde(default)]
    renderer: Vec<TomlRenderer>,
    #[serde(flatten)]
    config: TomlConfig,
}

impl TomlEntry {
    fn into_entry(self, origin: &Uri) -> anyhow::Result<Entry> {
        Ok(Entry {
            renderer: self
                .renderer
                .into_iter()
                .map(|r| r.into_renderer(origin))
                .collect::<Result<Vec<_>, _>>()?,
            config: self.config.into_config(origin)?,
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
    fn into_renderer(self, origin: &Uri) -> anyhow::Result<Renderer> {
        Ok(Renderer {
            source: self.source,
            config: self.config.into_config(origin)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct TomlConfig {
    #[serde(default)]
    include: Vec<Uri>,
    #[serde(flatten)]
    table: HashMap<String, Value>,
}

impl TomlConfig {
    fn into_config(
        self,
        origin: &Uri,
    ) -> anyhow::Result<HashMap<String, Value>> {
        let mut map = HashMap::new();
        for uri in self.include {
            let uri = origin.join(&uri)?;
            let s = uri.get()?;
            let config: TomlConfig = toml::from_str(&s)?;
            map.extend(config.into_config(&uri)?);
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

pub fn parse<P: AsRef<str>>(path: P) -> anyhow::Result<Entry> {
    let uri = Uri::from_str(path.as_ref())
        .ok()
        .context("Uri parsing error")?;
    let s = uri.get()?;
    let toml_entry: TomlEntry = toml::from_str(&s)?;
    toml_entry.into_entry(&uri)
}
