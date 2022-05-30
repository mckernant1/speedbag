use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(short, long, default_value_t = 8000)]
    pub port: u16,

    #[clap(short, long, arg_enum)]
    pub endpoints: Vec<Endpoint>,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Copy, Clone, PartialEq, Eq, ArgEnum, Debug)]
pub enum Endpoint {
    /// Add the load testing endpoints /rps-count and /rps-totals
    LoadTesting,
    /// Add the base / endpoint that just returns hello world
    Base,
}
