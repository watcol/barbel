use std::{fmt, fs::File, io::Read, path::PathBuf, str::FromStr};

use anyhow::Context;
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize,
};
use url::Url;

#[derive(Clone, Debug)]
pub enum Uri {
    Path(PathBuf),
    Url(Url),
}

impl FromStr for Uri {
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

impl Uri {
    pub fn get(&self) -> anyhow::Result<String> {
        Ok(match self {
            Uri::Path(path) => {
                let mut file = File::open(path)?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                buf
            }
            Uri::Url(url) => {
                ureq::request_url("GET", url).call()?.into_string()?
            }
        })
    }

    pub fn join(&self, other: &Uri) -> anyhow::Result<Uri> {
        match (self, other) {
            (_, Uri::Url(_)) => Ok(other.clone()),
            (Uri::Path(_), Uri::Path(p)) if p.is_absolute() => {
                Ok(other.clone())
            }
            (Uri::Path(base), Uri::Path(p)) => {
                let base = if base.is_file() {
                    base.parent().unwrap()
                } else {
                    base
                };
                Ok(Uri::Path(base.join(p)))
            }
            (Uri::Url(base), Uri::Path(p)) => Ok(Uri::Url(
                base.join(p.to_str().context("Illegal character.")?)?,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UriVisitor;

        impl<'de> Visitor<'de> for UriVisitor {
            type Value = Uri;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("path or url")
            }

            fn visit_str<E>(self, value: &str) -> Result<Uri, E>
            where
                E: de::Error,
            {
                Ok(Uri::from_str(value).unwrap())
            }
        }

        deserializer.deserialize_str(UriVisitor)
    }
}
