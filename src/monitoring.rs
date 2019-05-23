#![allow(non_snake_case)]

use std::sync::Arc;
use std::{thread, time};
use sysinfo::{System, SystemExt};

use crate::shared_state::SharedProcsState;

pub fn start_monitoring_thread(sharedState: Arc<SharedProcsState>) {
    let mut sys = System::new();
    let _mon_handle = thread::spawn(move || loop {
        // update the history of processes
        sys.refresh_processes();
        {
            let mut procsHistLocal = sharedState.procsHist.lock().unwrap();
            procsHistLocal.push_front(sys.get_process_list().clone());
            procsHistLocal.truncate(crate::HISTORY_KEEP_ITEMS_NB);
        } // unlock the mutex right away instead of waiting N milliseconds

        thread::sleep(time::Duration::from_millis(crate::MONITORING_PERIOD_MILLIS));
    });
}
