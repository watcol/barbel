mod parse;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(
        name = "FILE",
        help = "The entry configuration file",
        default_value = "barbel/main.toml"
    )]
    entry: PathBuf,
    #[structopt(
        short,
        long,
        help = "The output directory",
        default_value = "target"
    )]
    outdir: PathBuf,
}

fn main() {
    let opts = Opts::from_args();
    let entry = parse::user::parse(&opts.entry).unwrap();
    println!("{:?}", entry);
}
