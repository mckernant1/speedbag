mod args;
mod endpoints;
mod utils;

#[macro_use]
extern crate rocket;

use crate::args::args::Cli;
use crate::args::args::Endpoint::{All, Directory, LoadTesting};
use crate::endpoints::base::index;
use crate::endpoints::current_dir::current_dir;
use crate::endpoints::rps_testing::{rps_count, rps_totals};
use clap::Parser;
use log::log_enabled;
use log::Level::Info;

use rocket::Config;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};
use std::collections::HashMap;


use std::sync::{Arc, Mutex};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args: Cli = Cli::parse();
    let log_level = args.verbose.log_level_filter();

    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        ConfigBuilder::new().set_time_to_local(true).build(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )])
    .expect("Logger failed to initialize");

    if !log_enabled!(Info) {
        println!("Starting server http://127.0.0.1:{}", args.port);
    }

    let rocket = rocket::build().mount("/", routes![index]);

    let rocket = if args.endpoints.contains(&LoadTesting) || args.endpoints.contains(&All) {
        rocket
            .mount("/", routes![rps_count, rps_totals])
            .manage(Arc::new(Mutex::new(HashMap::<i64, u64>::new())))
    } else {
        rocket
    };

    let rocket = if args.endpoints.contains(&Directory) || args.endpoints.contains(&All) {
        rocket.mount("/", routes![current_dir])
    } else {
        rocket
    };

    let _r = rocket
        .manage(args.clone())
        .configure(Config {
            port: args.port,
            ..Config::default()
        })
        .launch()
        .await?;
    Ok(())
}
