#![allow(non_snake_case)]

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use sysinfo::{Pid, Process};

#[derive(Debug, Default)]
pub struct SharedProcsState {
    // the last N lists of processes
    // we use VecDeque as a buffer (append from front and truncate to max length)
    // since we append from the front, the first one is the most recent monitoring measurement
    // we keep the most recent `HISTORY_KEEP_ITEMS_NB: usize = 40` measurements and discard any older than that
    pub procsHist: Mutex<VecDeque<HashMap<Pid, Process>>>,
}
impl SharedProcsState {
    // countsHistByUser derives, from the state, the history of process counts, indexed by user
    pub fn countsHistByUser(&self) -> HashMap<u32, Vec<i32>> {
        self.procsHist
            .lock()
            .unwrap()
            .iter()
            .map(|processList| {
                processList
                    .iter()
                    .fold(HashMap::new(), |mut acc, (_, process)| {
                        let count = acc.entry(process.uid).or_insert(0);
                        *count += 1;
                        acc
                    })
            })
            // here we have a Vec<HashMap<user,procsCounts>> (history of {procsCount by user}),
            // we want a HashMap<user, Vec<procsCount>> ({history of procsCount} by user)
            .fold(HashMap::new(), |mut acc, procsByUserTimePoint| {
                // for each userID, push processCount into history Vec
                for (userID, count) in procsByUserTimePoint {
                    let countVecD = acc.entry(userID).or_insert(Vec::new());
                    countVecD.push(count);
                }
                acc
            })
    }
}
