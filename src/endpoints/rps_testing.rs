use chrono::{DateTime, Local, NaiveDateTime, Utc};
use itertools::Itertools;
use rocket::State;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[get("/rps-count")]
pub fn rps_count(rps_map: &State<Arc<Mutex<HashMap<i64, u64>>>>) -> String {
    let now = Utc::now().timestamp();
    let mut rps_map = rps_map.lock().unwrap();
    let counter = rps_map.get(&now).unwrap_or(&0_u64);
    let new_value = counter + 1;
    rps_map.insert(now, new_value);
    return format!("{}", new_value);
}

#[get("/rps-totals")]
pub fn rps_totals(rps_map: &State<Arc<Mutex<HashMap<i64, u64>>>>) -> String {
    return rps_map
        .lock()
        .unwrap()
        .iter()
        .sorted_by(|(ts, _), (ts2, _)| Ord::cmp(ts, ts2))
        .map(|(&timestamp, &counter)| {
            format!(
                "{}: {}",
                DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("could not parse timestamp"),
                    Utc,
                ).with_timezone(&Local),
                counter
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
}
