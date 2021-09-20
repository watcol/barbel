use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(
        name = "FILE",
        help = "The entry configuration file",
        default_value = "farbe/main.toml"
    )]
    entry: PathBuf,
    #[structopt(short, long, help = "The output directory", default_value = "target")]
    outdir: PathBuf,
}

#[derive(Deserialize, Debug)]
struct Entry {
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

fn main() {
    let opts = Opts::from_args();
    let toml = {
        let mut file = File::open(&opts.entry).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        s
    };
    let entry: Entry = toml::from_str(&toml).unwrap();
    println!("{:?}", entry);
}
