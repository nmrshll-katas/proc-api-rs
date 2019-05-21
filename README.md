# ps_api_rs

A Rust http API for exposing processes created by users

## Usage

`cargo run`

Then in a second terminal: `http localhost:3000/proc/groupby/owner`

## Requirements

- Rust 1.34.2+ (stable)
- for `http localhost:3000/proc/groupby/owner`: httpie

## Testing

Tests are missing for now. Just need some clarifications on what is acceptable behavior of the program before writing tests:

1. **Should the program ignore any processes that were already created (before launching the daemon), and focus only on the newly created ones ? Or count the total number of processes including the already created ones ? ( i.e. should I keep and serve a history of total count of processes, or should I keep one number (per user) of processes, which includes only the processes newer than the daemon itself ?)**

   Assumption: Only keep one count per user, for newly created processes.

2. **So far I have a program that reads existing processes every N seconds. Which means any process that gets created, then killed in that timeframe would not be seen by my program. Do I need to change my program so this gets taken into account ?**

   Assumption: It's fine to poll at intervals. Intervals should be short enough to catch most processes.

3. **If a process gets killed then relaunched, should I take it into account ?**

   Assumption: Yes. This counts as creating a process. And it should get counted again any number of times it gets killed then relaunched

4. **Should my program take itself into account in the counts ?**

   Assumption: No. Only count other processes.
