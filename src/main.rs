use std::path::PathBuf;

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

fn main() {
    let opts = Opts::from_args();
    println!("{:?}", opts);
}
