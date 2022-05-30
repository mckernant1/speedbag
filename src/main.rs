mod rps_testing;

#[macro_use]
extern crate rocket;

use rps_testing::{rps_count, rps_totals};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, rps_count, rps_totals])
        .manage(Arc::new(Mutex::new(HashMap::<i64, u64>::new())))
        .launch()
        .await?;
    Ok(())
}
