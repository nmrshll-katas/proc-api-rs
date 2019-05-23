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
