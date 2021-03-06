use once_cell::sync::Lazy;
use sha2::Digest;
use std::{
    env::var,
    fs::File,
    io::{Read, Write},
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
    use std::fmt::Write;
    let mut path = match Lazy::force(&CACHE_DIR) {
        Some(cache) => cache.join("sites"),
        None => return Ok(None),
    };
    let hash = {
        let mut hasher = sha2::Sha256::new();
        hasher.update(url.as_str().as_bytes());
        let bytes = hasher.finalize();
        let mut s = String::with_capacity(2 * bytes.len());
        for b in bytes {
            write!(s, "{:02x}", b)?;
        }
        s
    };
    path.push(hash);
    Ok(Some(path))
}

pub fn get_http(url: &Url) -> anyhow::Result<String> {
    if let Some(cache) = http_cache_path(url)? {
        if cache.exists() {
            get_file(cache)
        } else {
            let res = ureq::request_url("GET", url).call()?.into_string()?;
            std::fs::create_dir_all(cache.parent().unwrap())?;
            let mut file = File::create(cache)?;
            write!(file, "{}", res)?;
            Ok(res)
        }
    } else {
        Ok(ureq::request_url("GET", url).call()?.into_string()?)
    }
}
