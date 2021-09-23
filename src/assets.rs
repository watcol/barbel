use anyhow::Context;
use once_cell::sync::Lazy;
use std::{
    env::var,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use url::Url;

static CACHE_DIR: Lazy<Option<PathBuf>> = Lazy::new(|| {
    if cfg!(target_family = "unix") {
        if let Ok(s) = var("XDG_CACHE_HOME") {
            Some(PathBuf::from(s).join("barbel"))
        } else if let Ok(s) = var("HOME") {
            Some(PathBuf::from(s).join(".cache/barbel"))
        } else {
            None
        }
    } else if cfg!(target_family = "windows") {
        if let Ok(s) = var("LOCALAPPDATA") {
            Some(PathBuf::from(s).join("barbel"))
        } else if let Ok(s) = var("USERPROFILE") {
            Some(PathBuf::from(s).join(".barbel_cache"))
        } else {
            None
        }
    } else {
        None
    }
});

pub fn get_file<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn http_cache_path(url: &Url) -> anyhow::Result<Option<PathBuf>> {
    let mut path = match Lazy::force(&CACHE_DIR) {
        Some(cache) => cache.join("sites"),
        None => return Ok(None),
    };
    let authority = {
        let host = url.host_str().context("No host")?;
        match url.port() {
            Some(port) => format!("{}:{}", host, port),
            None => host.to_owned(),
        }
    };
    path.push(authority);
    if let Some(segments) = url.path_segments() {
        for segment in segments {
            path.push(segment);
        }
    }
    Ok(Some(path))
}

pub fn get_http(url: &Url) -> anyhow::Result<String> {
    let _cache_path = http_cache_path(url)?;
    // TODO: Cache load&store.
    Ok(ureq::request_url("GET", url).call()?.into_string()?)
}
