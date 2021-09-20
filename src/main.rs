use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(
        name = "FILE",
        help = "The entry configuration file",
        default_value = "farbe/main.toml"
    )]
    entry: PathBuf,
}

#[derive(Deserialize, Debug)]
struct Entry {
    #[serde(rename = "colorscheme")]
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    name: String,
    author: String,
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
