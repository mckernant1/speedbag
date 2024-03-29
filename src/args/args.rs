use clap::{ValueEnum, Parser};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// The port to run on
    #[clap(short, long, default_value_t = 8000)]
    pub port: u16,

    /// the endpoints on the test server you want to enable
    #[clap(short, long, value_enum)]
    pub endpoints: Vec<Endpoint>,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum Endpoint {
    /// Add all the endpoints
    All,
    /// Add the load testing endpoints /rps-count and /rps-totals
    LoadTesting,
    /// Add the base / endpoint that just returns hello world
    Base,
    /// Add the current directory as an endpoint on /cwd
    Directory,
}
