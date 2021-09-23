use std::{fmt, fs::File, io::Read, path::PathBuf, str::FromStr};

use anyhow::Context;
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize,
};
use url::Url;

#[derive(Clone, Debug)]
pub enum Address {
    Path(PathBuf),
    Url(Url),
}

impl FromStr for Address {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(Self::Url(Url::parse(s).map_err(|_| ())?))
        } else if s.starts_with("file://") {
            Ok(Self::Path(PathBuf::from(
                s.strip_prefix("file://").ok_or(())?,
            )))
        } else {
            Ok(Self::Path(PathBuf::from(s)))
        }
    }
}

impl Address {
    pub fn get(&self) -> anyhow::Result<String> {
        Ok(match self {
            Address::Path(path) => {
                let mut file = File::open(path)?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                buf
            }
            Address::Url(url) => {
                ureq::request_url("GET", url).call()?.into_string()?
            }
        })
    }

    pub fn join(&self, other: &Address) -> anyhow::Result<Address> {
        match (self, other) {
            (_, Address::Url(_)) => Ok(other.clone()),
            (Address::Path(_), Address::Path(p)) if p.is_absolute() => {
                Ok(other.clone())
            }
            (Address::Path(base), Address::Path(p)) => {
                let base = if base.is_file() {
                    base.parent().unwrap()
                } else {
                    base
                };
                Ok(Address::Path(base.join(p)))
            }
            (Address::Url(base), Address::Path(p)) => Ok(Address::Url(
                base.join(p.to_str().context("Illegal character.")?)?,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AddressVisitor;

        impl<'de> Visitor<'de> for AddressVisitor {
            type Value = Address;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("path or url")
            }

            fn visit_str<E>(self, value: &str) -> Result<Address, E>
            where
                E: de::Error,
            {
                Ok(Address::from_str(value).unwrap())
            }
        }

        deserializer.deserialize_str(AddressVisitor)
    }
}
