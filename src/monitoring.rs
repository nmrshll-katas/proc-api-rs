#![allow(non_snake_case)]

use std::sync::Arc;
use std::{thread, time};
use sysinfo::{System, SystemExt};

use crate::shared_state::SharedProcsState;

pub fn start_monitoring_thread(sharedState: Arc<SharedProcsState>) {
    let mut sys = System::new();
    thread::spawn(move || loop {
        // update the history of processes
        sys.refresh_processes();
        sharedState.pushProcessList(sys.get_process_list().clone());

        // mutex was released by pushProcessList, we can sleep() in this scope without keeping the mutex
        thread::sleep(time::Duration::from_millis(crate::MONITORING_PERIOD_MILLIS));
    });
}
