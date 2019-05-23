#![allow(non_snake_case)]
mod api;
mod monitoring;
mod shared_state;

#[macro_use]
extern crate serde_json;

use std::sync::Arc;

use shared_state::SharedProcsState;

// config
const HISTORY_KEEP_ITEMS_NB: usize = 40;
const MONITORING_PERIOD_MILLIS: u64 = 500;

fn main() {
    // shared state (across threads)
    let sharedState = Arc::new(SharedProcsState::default());
    // let mon_procsStore = sharedState.clone(); // copied reference to give to the thread closure

    // monitoring thread
    monitoring::start_monitoring_thread(sharedState.clone());

    // web server
    api::start_server(sharedState.clone());
}
