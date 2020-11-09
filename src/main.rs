use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!(",\n"))]
struct Args {
    /// C code entry point or Makefile
    #[clap(long, short)]
    entry: Option<PathBuf>,

    /// Config file path or directory containing a `polite-c.toml`
    #[clap(long, short)]
    config: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("The good time of day to you, gentle(wo)men of planet earth");

    println!("{:#?}", args);
}
