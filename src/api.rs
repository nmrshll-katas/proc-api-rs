#![allow(non_snake_case)]

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::sync::Arc;

use super::SharedProcsState;

pub fn start_server(sharedState: Arc<SharedProcsState>) {
    let router = move || {
        let router_procsStore = sharedState.clone();
        service_fn_ok(
            move |req: Request<Body>| match (req.method(), req.uri().path()) {
                // GET /proc/indexby/owner returns, indexed by owner, the history of process count
                (&Method::GET, "/proc/indexby/owner") => Response::new(Body::from(
                    json!({ "procs_by_owner": router_procsStore.countsHistByUser() }).to_string(),
                )),
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

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::rt::{self, Future};
    use hyper::Client;
    use std::thread;

    // NOTE: test setup is heavy and problematic (spinning up several server on the same port makes it impossible to run tests in parallel)
    // Ideally it would be nice to have something that takes the server router as a parameter and mocks every route,
    // which is callable directly in memory (doesn't need a port, doesn't rely on the OS networking)
    // I'm still figuring out how to do that properly in Rust (I will update this repo when I find a proper way to test, but I wanted to send this already)

    #[test]
    fn test_GET_proc_indexby_owner() {
        let state = Arc::new(SharedProcsState::default());
        thread::spawn(move || start_server(state.clone()));

        rt::run(rt::lazy(|| {
            let client = Client::new();
            let uri = "http://localhost:3000/proc/indexby/owner".parse().unwrap();

            client
                .get(uri)
                .map(|res| {
                    assert_eq!(res.status(), 200);
                })
                .map_err(|_| {
                    assert!(false); //fail test
                })
        }));
    }

    #[test]
    fn test_404() {
        let state = Arc::new(SharedProcsState::default());
        thread::spawn(move || start_server(state.clone()));

        rt::run(rt::lazy(|| {
            let client = Client::new();
            let uri = "http://localhost:3000/inexistent_path".parse().unwrap();

            client
                .get(uri)
                .map(|res| {
                    assert_eq!(res.status(), 404);
                })
                .map_err(|_| {
                    assert!(false); //fail test
                })
        }));
    }
}
