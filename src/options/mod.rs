extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Maximum number of tries
    #[clap(short, long, default_value_t = 6)]
    pub tries: u32,

    /// Length of the words
    #[clap(short, long, default_value_t = 5)]
    pub length: u32,

    /// Dictionary file to use
    #[clap(short, long, default_value = "jmena.txt")]
    pub dictionary: String,
}

pub fn parse() -> Args {
    Args::parse()
}
