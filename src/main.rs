#![allow(non_snake_case)]
mod api;
mod monitoring;
mod shared_state;

#[macro_use]
extern crate serde_json;

use shared_state::SharedProcsState;
use std::sync::Arc;

// config
const HISTORY_KEEP_ITEMS_NB: usize = 40;
const MONITORING_PERIOD_MILLIS: u64 = 500;

fn main() {
    // shared state (across threads)
    let sharedState = Arc::new(SharedProcsState::default());

    // monitoring thread
    monitoring::start_monitoring_thread(sharedState.clone());

    // web server
    api::start_server(sharedState.clone());
}
