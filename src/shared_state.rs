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
    pub fn pushProcessList(&self, procsList: HashMap<Pid, Process>) {
        let mut procsHistLocal = self.procsHist.lock().unwrap();
        procsHistLocal.push_front(procsList);
        procsHistLocal.truncate(crate::HISTORY_KEEP_ITEMS_NB);
    } // release mutex here

    pub fn getProcsHist(&self) -> VecDeque<HashMap<Pid, Process>> {
        self.procsHist.lock().unwrap().clone()
    } // release mutex here

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

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::{System, SystemExt};

    #[test]
    fn test_SharedProcsState() {
        let state = SharedProcsState::default();
        let sys = System::new();

        // push some data into the state
        for _ in 1..=5 {
            state.pushProcessList(sys.get_process_list().clone())
        }

        // check data is inserted correctly
        assert_eq!(state.getProcsHist().len(), 5);
        // check the shape of countsHistByUser()
        for (userID, userProcsHist) in state.countsHistByUser() {
            assert!(userID <= 65535);
            assert_eq!(userProcsHist.len(), 5)
        }

        // push some more data into the state
        for _ in 1..=5 {
            state.pushProcessList(sys.get_process_list().clone())
        }

        // check data is inserted correctly
        assert_eq!(state.getProcsHist().len(), 10);
        // check the shape of countsHistByUser()
        for (userID, userProcsHist) in state.countsHistByUser() {
            assert!(userID <= 65535);
            assert_eq!(userProcsHist.len(), 10)
        }

        // push too much data into the state. We'll check the state keeps only the 40 past values
        for _ in 1..=100 {
            state.pushProcessList(sys.get_process_list().clone())
        }

        // check data is inserted correctly
        assert_eq!(crate::HISTORY_KEEP_ITEMS_NB, 40);
        assert_eq!(state.getProcsHist().len(), crate::HISTORY_KEEP_ITEMS_NB);
        // check the shape of countsHistByUser()
        for (userID, userProcsHist) in state.countsHistByUser() {
            assert!(userID <= 65535);
            assert_eq!(userProcsHist.len(), crate::HISTORY_KEEP_ITEMS_NB)
        }
    }
}
