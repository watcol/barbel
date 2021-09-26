use anyhow::Context;
use serde::Deserialize;
use std::{
    collections::{hash_map::Entry as HashMapEntry, HashMap},
    str::FromStr,
};
use toml::{map::Entry as MapEntry, Value};

use super::address::Address;

#[derive(Clone, Debug, Deserialize)]
struct TomlEntry {
    #[serde(default)]
    renderer: Vec<TomlRenderer>,
    #[serde(flatten)]
    config: TomlConfig,
}

impl TomlEntry {
    fn into_entry(self, origin: &Address) -> anyhow::Result<Entry> {
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
    fn into_renderer(self, origin: &Address) -> anyhow::Result<Renderer> {
        Ok(Renderer {
            source: self.source,
            config: self.config.into_config(origin)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct TomlConfig {
    #[serde(default)]
    include: Vec<Address>,
    #[serde(flatten)]
    table: HashMap<String, Value>,
}

impl TomlConfig {
    fn into_config(
        self,
        origin: &Address,
    ) -> anyhow::Result<HashMap<String, Value>> {
        let mut map = HashMap::new();
        for address in self.include {
            let address = origin.join(&address)?;
            let s = address.get()?;
            let config: TomlConfig = toml::from_str(&s)?;
            hashmap_merge(&mut map, config.into_config(&address)?);
        }
        hashmap_merge(&mut map, self.table);
        Ok(map)
    }
}

fn hashmap_merge(
    origin: &mut HashMap<String, Value>,
    append: HashMap<String, Value>,
) {
    for (k, v) in append {
        match origin.entry(k) {
            HashMapEntry::Occupied(mut e) => value_merge(e.get_mut(), v),
            HashMapEntry::Vacant(e) => {
                e.insert(v);
            }
        }
    }
}

fn value_merge(origin: &mut Value, append: Value) {
    match (origin, append) {
        (Value::Table(orig), Value::Table(a)) => {
            for (k, v) in a {
                match orig.entry(k) {
                    MapEntry::Vacant(e) => {
                        e.insert(v);
                    }
                    MapEntry::Occupied(mut e) => value_merge(e.get_mut(), v),
                }
            }
        }
        (Value::Array(orig), Value::Array(mut a)) => {
            orig.append(&mut a);
        }
        (orig, a) => {
            *orig = a;
        }
    }
}

/// The entry of user configurations
#[derive(Clone, Debug)]
pub struct Entry {
    pub renderer: Vec<Renderer>,
    pub config: HashMap<String, Value>,
}

/// Renderer configurations
#[derive(Clone, Debug)]
pub struct Renderer {
    pub source: String,
    pub config: HashMap<String, Value>,
}

/// Parse a toml file into `Entry`.
pub fn parse<P: AsRef<str>>(path: P) -> anyhow::Result<Entry> {
    let address = Address::from_str(path.as_ref())
        .ok()
        .context("Address parsing error")?;
    let s = address.get()?;
    let toml_entry: TomlEntry = toml::from_str(&s)?;
    toml_entry.into_entry(&address)
}
