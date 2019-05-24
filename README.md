# ps_api_rs

A Rust http API for exposing processes created by users

## Usage

`cargo run`

Then in a second terminal: `http localhost:3000/proc/indexby/owner`

## Requirements

- Rust 1.34.2+ (stable)
- for `http localhost:3000/proc/indexby/owner`: httpie

## Testing

`cargo test`
