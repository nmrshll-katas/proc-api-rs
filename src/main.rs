#![allow(non_snake_case)]

#[macro_use]
extern crate serde_json;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use sysinfo::{System, SystemExt};

fn main() {
    // shared state (across threads)
    let procsByUserStore = Arc::new(Mutex::new(HashMap::new()));

    // monitoring process info
    let mut sys = System::new();
    let procsByUserStoreClone = Arc::clone(&procsByUserStore); // give a new cloned reference to the closure
    let _handle = thread::spawn(move || loop {
        sys.refresh_processes();
        let procsByUser =
            sys.get_process_list()
                .iter()
                .fold(HashMap::new(), |mut acc, (_, process)| {
                    let count = acc.entry(process.gid).or_insert(0);
                    *count += 1;
                    acc
                });
        {
            let mut procsByUserStoreLocal = procsByUserStoreClone.lock().unwrap(); // TODO: print error instead of panic
            *procsByUserStoreLocal = procsByUser;
        } // unlock the mutex right away instead of waiting 5 seconds

        thread::sleep(time::Duration::from_secs(5));
    });

    // web server
    //
    // take ownership of the shared state
    let router = move || {
        let procsByUserStoreClone = Arc::clone(&procsByUserStore);
        service_fn_ok(
            move |req: Request<Body>| match (req.method(), req.uri().path()) {
                // GET /proc/groupby/owner returns the number of processes for each owner
                (&Method::GET, "/proc/groupby/owner") => {
                    let procsByUserStoreLocal = procsByUserStoreClone.lock().unwrap();
                    Response::new(Body::from(
                        json!({ "procs_by_owner": *procsByUserStoreLocal }).to_string(),
                    ))
                }
                // catch-all default case. Returns a 404 status + error body.
                (_, _) => {
                    let mut res = Response::new(Body::from(
                        json!({ "status": "error","code":StatusCode::NOT_FOUND.to_string(),"message":"not found" }).to_string(),
                    ));
                    *res.status_mut() = StatusCode::NOT_FOUND;
                    res
                }
            },
        )
    };
    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr)
        .serve(router)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("running server on {}", addr.to_string());
    hyper::rt::run(server);
}
